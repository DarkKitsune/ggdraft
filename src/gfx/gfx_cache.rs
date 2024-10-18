use std::{
    any::{Any, TypeId},
    collections::HashMap,
    path::Path,
    rc::Rc,
};

use anyhow::Result;

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

pub struct GfxCache {
    objects: HashMap<TypeId, HashMap<String, Box<dyn Any>>>,
}

impl GfxCache {
    /// Create a new GfxCache
    /// # Safety
    /// This function is unsafe because it should only be used on the main thread.
    pub(crate) unsafe fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    /// Insert a new object into the cache.
    /// Returns the old object if it exists.
    pub fn insert<T: Any>(&mut self, key: impl Into<String>, value: T) -> Option<T> {
        // Get the type id of the value.
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id.
        let map = self.objects.entry(type_id).or_insert_with(HashMap::new);

        // Insert the value into the hashmap.
        map.insert(key.into(), Box::new(value))
            .and_then(|v| v.downcast().ok())
            .map(|v| *v)
    }

    /// Create a new vertex layout in the cache.
    /// The vertex layout is created using the given function.
    /// The actual type in the cache is `Rc<VertexLayout>`.
    pub fn create_vertex_layout(
        &mut self,
        key: impl Into<String>,
        f: impl FnOnce() -> VertexLayout,
    ) {
        // Create the vertex layout
        let vertex_layout = Rc::new(f());

        // Insert the vertex layout into the cache
        self.insert(key, vertex_layout);
    }

    /// Returns the vertex layout if it already exists.
    /// Otherwise, creates a new vertex layout from the given function,
    /// and inserts it into the cache.
    /// The actual type in the cache is `Rc<VertexLayout>`.
    pub fn inline_vertex_layout(
        &mut self,
        key: impl AsRef<str>,
        f: impl FnOnce() -> VertexLayout,
    ) -> &VertexLayout {
        let key = key.as_ref();

        if self.contains::<Rc<VertexLayout>>(key) {
            // Get the vertex layout if it already exists
            self.get(key).unwrap()
        } else {
            // Create the vertex layout if it does not exist
            self.create_vertex_layout(key, f);
            self.get(key).unwrap()
        }
    }

    /// Create a new buffer in the cache.
    pub fn create_buffer_from_slice<T: 'static>(&mut self, key: impl Into<String>, data: &[T]) {
        // Create the buffer
        let buffer = unsafe { Buffer::__from_slice(data, None) };

        // Insert the buffer into the cache
        self.insert(key, buffer);
    }

    /// Returns the buffer if it already exists.
    /// Otherwise, creates a new buffer from the given function,
    /// and inserts it into the cache.
    pub fn inline_buffer<T: 'static>(
        &mut self,
        key: impl AsRef<str>,
        f: impl FnOnce() -> Vec<T>,
    ) -> &Buffer<T> {
        let key = key.as_ref();

        if self.contains::<Buffer<T>>(key) {
            // Get the buffer if it already exists
            self.get(key).unwrap()
        } else {
            // Create the buffer if it does not exist
            self.create_buffer_from_slice(key, &f());
            self.get(key).unwrap()
        }
    }

    /// Create a new texture in the cache from the given file path.
    /// Returns an error if the file could not be loaded.
    // TODO: Implement LODs
    pub fn create_texture_from_file(
        &mut self,
        key: impl Into<String>,
        texture_type: TextureType,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let path = path.as_ref();

        // Get the file name from the path without the extension
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid path {:?}", path))?;

        // Open the image file
        let image = image::open(path)
            .map_err(|e| anyhow::anyhow!("Failed to open image file {:?}: {:?}", path, e))?;

        // Create the texture
        let texture = unsafe { Texture::__from_image(name, texture_type, &[image])? };

        // Insert the texture into the cache
        self.insert(key, texture);

        Ok(())
    }

    /// Returns the texture if it already exists.
    /// Otherwise, creates a new texture from the given path,
    /// and inserts it into the cache.
    pub fn inline_texture_view_from_file(
        &mut self,
        key: impl AsRef<str>,
        texture_type: TextureType,
        path: impl AsRef<Path>,
    ) -> Result<TextureView> {
        let key = key.as_ref();

        if self.contains::<Texture>(key) {
            // Get the texture if it already exists
            Ok(self.get_texture_view(key).unwrap())
        } else {
            // Create the texture if it does not exist
            self.create_texture_from_file(key, texture_type, path)?;
            Ok(self.get_texture_view(key).unwrap())
        }
    }

    /// Create a new mesh in the cache from the given vertex list.
    pub fn create_mesh<'a>(
        &mut self,
        key: impl Into<String>,
        vertex_layout_key: impl AsRef<str>,
        vertex_list: impl IntoVertexList<'a>,
    ) {
        // Get the vertex layout from the cache
        let vertex_layout = self.get_vertex_layout(vertex_layout_key).unwrap();

        // Get the vertex list
        let vertex_list = vertex_list.into_vertex_list(vertex_layout.clone());

        // Create the vertex buffer
        let vertex_buffer =
            unsafe { Buffer::__from_slice(vertex_list.vertex_data(), Some(vertex_layout)) };

        // Create the index buffer
        let index_buffer = unsafe { Buffer::__from_slice(vertex_list.indices(), None) };

        // Create the mesh in the cache
        self.insert(key, Mesh::new(vertex_buffer, index_buffer));
    }

    /// Returns the mesh if it already exists.
    /// Otherwise, creates a new mesh from the given function,
    /// and inserts it into the cache.
    pub fn inline_mesh<'a, V: IntoVertexList<'a>>(
        &mut self,
        key: impl AsRef<str>,
        vertex_layout_key: impl AsRef<str>,
        f: impl FnOnce() -> V,
    ) -> &Mesh {
        let key = key.as_ref();

        if self.contains::<Mesh>(key) {
            // Get the mesh if it already exists
            self.get(key).unwrap()
        } else {
            // Create the mesh if it does not exist
            self.create_mesh(key, vertex_layout_key, f());
            self.get(key).unwrap()
        }
    }

    /// Create a new program in the cache using the given input layout.
    /// The program's vertex and fragment shaders are generated using the callbacks.
    pub fn create_program_vertex_fragment(
        &mut self,
        key: impl Into<String>,
        input_layout_key: impl AsRef<str>,
        vertex: impl FnOnce(&ShaderInputs, &mut ShaderParameters, &mut ShaderOutputs) -> Result<()>,
        fragment: impl FnOnce(&ShaderInputs, &mut ShaderParameters, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<()> {
        // Get the input layout from the cache
        let input_layout = self
            .get::<InputLayout>(input_layout_key.as_ref())
            .ok_or_else(|| {
                anyhow::anyhow!("Input layout not found: {}", input_layout_key.as_ref())
            })?;

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
        self.insert(key, program);

        Ok(())
    }

    /// Returns the program if it already exists.
    /// Otherwise, creates a new program from the given functions, and inserts it into the cache.
    /// The program's vertex and fragment shaders are generated using the callbacks.
    pub fn inline_program_vertex_fragment(
        &mut self,
        key: impl AsRef<str>,
        input_layout_key: impl AsRef<str>,
        vertex: impl FnOnce(&ShaderInputs, &mut ShaderParameters, &mut ShaderOutputs) -> Result<()>,
        fragment: impl FnOnce(&ShaderInputs, &mut ShaderParameters, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<&Program> {
        let key = key.as_ref();

        if self.contains::<Program>(key) {
            // Get the program if it already exists
            Ok(self.get(key).unwrap())
        } else {
            // Create the program if it does not exist
            self.create_program_vertex_fragment(key, input_layout_key, vertex, fragment)?;
            Ok(self.get(key).unwrap())
        }
    }

    /// Create a new input layout in the cache from the given vertex layout.
    pub fn create_input_layout_from_vertex_layout(
        &mut self,
        key: impl Into<String>,
        vertex_layout_key: impl AsRef<str>,
    ) {
        // Get the vertex layout from the cache
        let vertex_layout = self.get_vertex_layout(vertex_layout_key).unwrap();

        // Create the input layout
        let input_layout = unsafe { InputLayout::__from_vertex_layout(vertex_layout) };
        self.insert(key, input_layout);
    }

    /// Returns the input layout if it already exists.
    /// Otherwise, creates a new input layout from the given vertex layout,
    /// and inserts it into the cache.
    pub fn inline_input_layout_from_vertex_layout(
        &mut self,
        key: impl AsRef<str>,
        vertex_layout_key: impl AsRef<str>,
    ) -> &InputLayout {
        let key = key.as_ref();

        if self.contains::<InputLayout>(key) {
            // Get the input layout if it already exists
            self.get(key).unwrap()
        } else {
            // Create the input layout if it does not exist
            self.create_input_layout_from_vertex_layout(key, vertex_layout_key);
            self.get(key).unwrap()
        }
    }

    /// Get an object from the cache.
    pub fn get<T: Any>(&self, key: impl AsRef<str>) -> Option<&T> {
        let key = key.as_ref();

        // Get the type id of the value
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id
        let map = self.objects.get(&type_id)?;

        // Get the value from the hashmap
        map.get(key).and_then(|v| v.downcast_ref())
    }

    /// Check if an object exists in the cache.
    pub fn contains<T: Any>(&self, key: impl AsRef<str>) -> bool {
        self.get::<T>(key).is_some()
    }

    /// Remove an object from the cache.
    /// Returns the removed object if it exists.
    pub fn remove<T: Any>(&mut self, key: impl AsRef<str>) -> Option<&T> {
        let key = key.as_ref();

        // Get the type id of the value
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id
        let map = self.objects.get_mut(&type_id)?;

        // Remove the value from the hashmap
        map.remove(key).and_then(|v| v.downcast().ok()).map(|v| *v)
    }

    /// Get a vertex layout from the cache.
    /// Returns an error if the vertex layout does not exist.
    pub fn get_vertex_layout(&self, key: impl AsRef<str>) -> Result<Rc<VertexLayout>> {
        let key = key.as_ref();
        self.get::<Rc<VertexLayout>>(key)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Vertex layout not found: {}", key))
    }

    /// Get a buffer from the cache.
    /// Returns an error if the buffer does not exist.
    pub fn get_buffer<T: 'static>(&self, key: impl AsRef<str>) -> Result<&Buffer<T>> {
        let key = key.as_ref();
        self.get::<Buffer<T>>(key)
            .ok_or_else(|| anyhow::anyhow!("Buffer not found: {}", key))
    }

    /// Get a `TextureView` of a texture with the given key from the cache.
    /// Returns an error if the texture does not exist.
    pub fn get_texture_view(&self, key: impl AsRef<str>) -> Result<TextureView> {
        let key = key.as_ref();
        self.get::<Texture>(key)
            .map(|t| t.view())
            .ok_or_else(|| anyhow::anyhow!("Texture not found: {}", key))
    }
}
