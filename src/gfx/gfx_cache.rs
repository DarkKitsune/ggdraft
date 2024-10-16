use std::{
    any::{Any, TypeId},
    collections::HashMap, rc::Rc,
};

use anyhow::Result;

use super::{
    buffer::Buffer, input_layout::InputLayout, mesh::Mesh, program::Program, shader::{Shader, ShaderStage}, shader_gen::{shader_inputs::ShaderInputs, shader_outputs::ShaderOutputs}, vertex_layout::VertexLayout, vertex_list::VertexList
};

pub struct GfxCache {
    objects: HashMap<TypeId, HashMap<String, Box<dyn Any>>>,
}

impl GfxCache {
    /// Create a new GfxCache
    pub(crate) fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    /// Insert a new object into the cache.
    /// Returns the old object if it exists.
    pub fn insert<T: Any>(&mut self, key: impl Into<String>, value: T) -> Option<T> {
        // Get the type id of the value
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id
        let map = self.objects.entry(type_id).or_insert_with(HashMap::new);

        // Insert the value into the hashmap
        map.insert(key.into(), Box::new(value))
            .and_then(|v| v.downcast().ok())
            .map(|v| *v)
    }

    /// Create a new buffer with the given length in the cache.
    pub fn create_buffer_from_slice<T: 'static>(&mut self, key: impl Into<String>, data: &[T]) {
        // Create the buffer
        let buffer = Buffer::__from_slice(data, None);

        // Insert the buffer into the cache
        self.insert(key, buffer);
    }

    /// Create a `Mesh` in the cache from the given vertex list.
    pub fn create_mesh(&mut self, key: impl Into<String>, vertex_layout: Rc<VertexLayout>, vertex_list: &VertexList) {
        // Create the vertex buffer
        let vertex_buffer = Buffer::__from_slice(vertex_list.vertex_data(), Some(vertex_layout));

        // Create the index buffer
        let index_buffer = Buffer::__from_slice(vertex_list.indices(), None);

        // Create the mesh in the cache
        self.insert(key, Mesh::new(vertex_buffer, index_buffer));
    }

    /// Create a new program in the cache using the given input layout.
    /// The program's vertex and fragment shaders are generated using the callbacks.
    pub fn create_program_vertex_fragment(
        &mut self,
        key: impl Into<String>,
        input_layout_key: impl AsRef<str>,
        vertex: impl FnOnce(&ShaderInputs, &mut ShaderOutputs) -> Result<()>,
        fragment: impl FnOnce(&ShaderInputs, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<()> {
        // Get the input layout from the cache
        let input_layout = self
            .get::<InputLayout>(input_layout_key.as_ref())
            .ok_or_else(|| {
                anyhow::anyhow!("Input layout not found: {}", input_layout_key.as_ref())
            })?;

        // Generate the vertex and fragment shaders
        let (vertex_code, fragment_code) =
            input_layout.generate_vertex_fragment_shaders(vertex, fragment)?;
        let vertex_shader = Shader::__new(ShaderStage::Vertex, &vertex_code)?;
        let fragment_shader = Shader::__new(ShaderStage::Fragment, &fragment_code)?;

        // Create the program from the shaders
        let program = Program::__new(&[vertex_shader, fragment_shader])?;

        // Insert the program into the cache
        self.insert(key, program);

        Ok(())
    }

    /// Create a new input layout in the cache from the given vertex layout.
    pub fn create_input_layout_from_vertex_layout(
        &mut self,
        key: impl Into<String>,
        vertex_layout: Rc<VertexLayout>,
    ) {
        let input_layout = InputLayout::__from_vertex_layout(vertex_layout);
        self.insert(key, input_layout);
    }

    /// Get an object from the cache.
    pub fn get<T: Any>(&self, key: &str) -> Option<&T> {
        // Get the type id of the value
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id
        let map = self.objects.get(&type_id)?;

        // Get the value from the hashmap
        map.get(key).and_then(|v| v.downcast_ref())
    }

    /// Remove an object from the cache.
    /// Returns the removed object if it exists.
    pub fn remove<T: Any>(&mut self, key: &str) -> Option<T> {
        // Get the type id of the value
        let type_id = TypeId::of::<T>();

        // Get the hashmap for the type id
        let map = self.objects.get_mut(&type_id)?;

        // Remove the value from the hashmap
        map.remove(key).and_then(|v| v.downcast().ok()).map(|v| *v)
    }
}
