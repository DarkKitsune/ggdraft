use std::{
    any::{Any, TypeId},
    collections::HashMap,
    path::Path,
    rc::Rc,
};

use anyhow::Result;
use ggutil::prelude::*;

use crate::app::app_prelude::ShaderParameters;

use super::{
    buffer::Buffer,
    input_layout::InputLayout,
    mesh::Mesh,
    program::Program,
    shader::{Shader, ShaderStage},
    shader_gen::{shader_inputs::ShaderInputs, shader_outputs::ShaderOutputs},
    texture::{Texture, TextureType, TextureView},
    vertex_layout::VertexLayout,
    vertex_list::IntoVertexList,
};

/// A handle pointing to an object in the `GfxCache`.
pub type CacheHandle = Handle;

/// A cache for storing graphics objects between renders.
pub struct GfxCache {
    objects: HashMap<TypeId, HandleMap<Box<dyn Any>>>,
    names: HashMap<String, CacheHandle>,
}

impl GfxCache {
    /// Create a new GfxCache
    /// # Safety
    /// This function is unsafe because it should only be used on the main thread.
    pub(crate) unsafe fn new() -> Self {
        Self {
            objects: HashMap::new(),
            names: HashMap::new(),
        }
    }

    /// Insert a new object into the cache.
    pub fn insert<T: Any>(&mut self, value: T) -> CacheHandle {
        // Get the type id of the value.
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id.
        let map = self.objects.entry(type_id).or_insert_with(HandleMap::new);

        // Insert the value into the hashmap.
        map.insert(Box::new(value))
    }

    /// Get an object from the cache.
    pub fn get<T: Any>(&self, name_or_handle: impl CacheRef) -> Option<&T> {
        // Get the type id of the value
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id
        let map = self.objects.get(&type_id)?;

        // Get the value from the hashmap
        map.get(&name_or_handle.handle(self))
            .and_then(|v| v.downcast_ref())
    }

    /// Set the name of an object in the cache.
    /// This allows the object to be retrieved by name.
    pub fn set_name(&mut self, handle_or_old_name: impl CacheRef, new_name: impl Into<String>) {
        self.names
            .insert(new_name.into(), handle_or_old_name.handle(self));
    }

    /// Get an object's handle by name.
    pub fn get_handle_of(&self, name: impl AsRef<str>) -> Option<&CacheHandle> {
        self.names.get(name.as_ref())
    }

    /// Check if an object exists in the cache.
    pub fn contains<T: Any>(&self, name_or_handle: impl CacheRef) -> bool {
        self.get::<T>(name_or_handle).is_some()
    }

    /// Remove an object from the cache.
    /// Returns the removed object if it exists.
    pub fn remove<T: Any>(&mut self, name_or_handle: impl CacheRef) -> Option<T> {
        // Get the type id of the value.
        let type_id = TypeId::of::<T>();

        // Get the handle of the value.
        let handle = name_or_handle.handle(self);

        // Get the hashmap for the type id.
        let map = self.objects.get_mut(&type_id)?;

        // Remove the name from the hashmap.
        self.names.retain(|_, h| *h != handle);

        // Remove the value from the hashmap.
        map.remove(&handle)
            .and_then(|v| v.downcast().ok().map(|v| *v))
    }

    /// Create a new vertex layout in the cache.
    /// The vertex layout is created using the given function.
    /// The actual type in the cache is `Rc<VertexLayout>`.
    pub fn create_vertex_layout(
        &mut self,
        name: Option<impl Into<String>>,
        f: impl FnOnce(VertexLayout) -> VertexLayout,
    ) -> CacheHandle {
        // Create the vertex layout.
        let vertex_layout = Rc::new(f(unsafe { VertexLayout::__new() }));

        // Validate the vertex layout.
        vertex_layout.validate().unwrap();

        // Insert the vertex layout into the cache.
        let handle = self.insert(vertex_layout);

        // Set the name of the vertex layout.
        if let Some(name) = name {
            self.set_name(handle.clone(), name);
        }

        handle
    }

    /// Get a vertex layout from the cache.
    pub fn get_vertex_layout(&self, name_or_handle: impl CacheRef) -> Option<&Rc<VertexLayout>> {
        self.get::<Rc<VertexLayout>>(name_or_handle)
    }

    /// Create a new buffer in the cache.
    pub fn create_buffer_from_slice<T: 'static>(
        &mut self,
        name: Option<impl Into<String>>,
        data: &[T],
    ) -> CacheHandle {
        // Create the buffer.
        let buffer = unsafe { Buffer::__from_slice(data, None) };

        // Insert the buffer into the cache.
        let handle = self.insert(buffer);

        // Set the name of the buffer.
        if let Some(name) = name {
            self.set_name(handle.clone(), name);
        }

        handle
    }

    /// Get a buffer from the cache.
    pub fn get_buffer<T: 'static>(&self, name_or_handle: impl CacheRef) -> Option<&Buffer<T>> {
        self.get::<Buffer<T>>(name_or_handle)
    }

    /// Create a new texture in the cache from the given file path.
    /// Returns an error if the file could not be loaded.
    // TODO: Implement LODs
    pub fn create_texture_from_file(
        &mut self,
        name: Option<impl Into<String>>,
        texture_type: TextureType,
        path: impl AsRef<Path>,
    ) -> Result<CacheHandle> {
        let path = path.as_ref();

        // Get the file name from the path without the extension.
        let name = name.map(|name| name.into()).unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "texture".to_string())
        });

        // Open the image file.
        let image = image::open(path)
            .map_err(|e| anyhow::anyhow!("Failed to open image file {:?}: {:?}", path, e))?;

        // Create the texture.
        let texture = unsafe { Texture::__from_image(&name, texture_type, &[image])? };

        // Insert the texture into the cache.
        let handle = self.insert(texture);

        // Set the name of the texture.
        self.set_name(handle.clone(), name);

        Ok(handle)
    }

    /// Get a `TextureView` from the cache.
    pub fn get_texture_view(&self, name_or_handle: impl CacheRef) -> Option<TextureView> {
        let texture = self.get::<Texture>(name_or_handle)?;
        Some(texture.view())
    }

    /// Create a new mesh in the cache from the given vertex list.
    pub fn create_mesh<'a>(
        &mut self,
        name: Option<impl Into<String>>,
        vertex_layout: impl CacheRef,
        vertex_list: impl IntoVertexList<'a>,
    ) -> CacheHandle {
        // Get the vertex layout from the cache.
        let vertex_layout = self.get_vertex_layout(vertex_layout).unwrap();

        // Get the vertex list.
        let vertex_list = vertex_list.into_vertex_list(vertex_layout.clone());

        // Create the vertex buffer.
        let vertex_buffer =
            unsafe { Buffer::__from_slice(vertex_list.vertex_data(), Some(vertex_layout.clone())) };

        // Create the index buffer.
        let index_buffer = unsafe { Buffer::__from_slice(vertex_list.indices(), None) };

        // Create the mesh into the cache.
        let handle = self.insert(Mesh::new(vertex_buffer, index_buffer));

        // Set the name of the mesh.
        if let Some(name) = name {
            self.set_name(handle.clone(), name);
        }

        handle
    }

    /// Get a `Mesh` from the cache.
    pub fn get_mesh(&self, name_or_handle: impl CacheRef) -> Option<&Mesh> {
        self.get::<Mesh>(name_or_handle)
    }

    /// Create a new program in the cache using the given input layout.
    /// The program's vertex and fragment shaders are generated using the callbacks.
    pub fn create_program_vertex_fragment(
        &mut self,
        name: Option<impl Into<String>>,
        input_layout: impl CacheRef,
        vertex: impl FnOnce(&ShaderInputs, &mut ShaderParameters, &mut ShaderOutputs) -> Result<()>,
        fragment: impl FnOnce(&ShaderInputs, &mut ShaderParameters, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<CacheHandle> {
        // Get the input layout from the cache
        let input_layout = self
            .get::<InputLayout>(input_layout)
            .ok_or_else(|| anyhow::anyhow!("Input layout not found"))?;

        // Generate the vertex and fragment shaders
        let (vertex_code, vertex_parameters, fragment_code, fragment_parameters) =
            input_layout.generate_vertex_fragment_shaders(vertex, fragment)?;
        let vertex_shader =
            unsafe { Shader::__new(ShaderStage::Vertex, &vertex_code, vertex_parameters)? };
        let fragment_shader =
            unsafe { Shader::__new(ShaderStage::Fragment, &fragment_code, fragment_parameters)? };

        // Create the program from the shaders
        let program = unsafe { Program::__new(&[vertex_shader, fragment_shader])? };

        // Insert the program into the cache
        let handle = self.insert(program);

        // Set the name of the program
        if let Some(name) = name {
            self.set_name(handle.clone(), name);
        }

        Ok(handle)
    }

    /// Get a `Program` from the cache.
    pub fn get_program(&self, name_or_handle: impl CacheRef) -> Option<&Program> {
        self.get::<Program>(name_or_handle)
    }

    /// Create a new input layout in the cache from the given vertex layout.
    pub fn create_input_layout_from_vertex_layout(
        &mut self,
        name: Option<impl Into<String>>,
        vertex_layout: impl CacheRef,
    ) -> CacheHandle {
        // Get the vertex layout from the cache
        let vertex_layout = self.get_vertex_layout(vertex_layout).unwrap();

        // Create the input layout
        let input_layout = unsafe { InputLayout::__from_vertex_layout(vertex_layout.clone()) };

        // Insert the input layout into the cache
        let handle = self.insert(input_layout);

        // Set the name of the input layout
        if let Some(name) = name {
            self.set_name(handle.clone(), name);
        }

        handle
    }

    /// Get an `InputLayout` from the cache.
    pub fn get_input_layout(&self, name_or_handle: impl CacheRef) -> Option<&InputLayout> {
        self.get::<InputLayout>(name_or_handle)
    }
}

/// Trait for types that point to an object in the `GfxCache`.
pub trait CacheRef: Clone {
    /// Get the equivalent `CacheHandle` from the cache.
    fn handle(self, cache: &GfxCache) -> CacheHandle;
}

// Implement `CacheRef` for `CacheHandle`.
impl CacheRef for CacheHandle {
    fn handle(self, _: &GfxCache) -> CacheHandle {
        self
    }
}

// Implement `CacheRef` for `&CacheHandle`.
impl CacheRef for &CacheHandle {
    fn handle(self, _: &GfxCache) -> CacheHandle {
        self.clone()
    }
}

// Implement `CacheRef` for `String`.
impl CacheRef for String {
    fn handle(self, cache: &GfxCache) -> CacheHandle {
        cache.get_handle_of(&self).unwrap().clone()
    }
}

// Implement `CacheRef` for `&str`.
impl CacheRef for &str {
    fn handle(self, cache: &GfxCache) -> CacheHandle {
        cache.get_handle_of(self).unwrap().clone()
    }
}
