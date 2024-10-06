use std::{any::{Any, TypeId}, collections::HashMap};

use super::{buffer::Buffer, input_layout::InputLayout, program::Program, shader::{Shader, ShaderStage}, vertex_layout::VertexLayout, vertex_list::VertexList};

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
        let map = self.objects
            .entry(type_id)
            .or_insert_with(HashMap::new);

        // Insert the value into the hashmap
        map.insert(key.into(), Box::new(value)).and_then(|v| v.downcast().ok())
            .map(|v| *v)
    }

    /// Create a new buffer with the given length in the cache.
    pub fn create_vertex_buffer(&mut self, key: impl Into<String>, vertex_list: &VertexList) {
        let buffer = Buffer::__from_slice(vertex_list.data(), Some(vertex_list.layout().clone()));
        self.insert(key, buffer);
    }

    /// Create a new buffer with the given length in the cache.
    pub fn create_buffer_from_slice<T: 'static>(&mut self, key: impl Into<String>, data: &[T]) {
        let buffer = Buffer::__from_slice(data, None);
        self.insert(key, buffer);
    }

    /// Create a new program in the cache.
    pub fn create_program_vertex_fragment(&mut self, key: impl Into<String>, vertex_shader: &str, fragment_shader: &str) {
        let vertex_shader = Shader::__new(vertex_shader, ShaderStage::Vertex).unwrap();
        let fragment_shader = Shader::__new(fragment_shader, ShaderStage::Fragment).unwrap();
        let program = Program::__new(&[vertex_shader, fragment_shader]).unwrap();
        self.insert(key, program);
    }

    /// Create a new program in the cache with default settings.
    /// The only vertex inputs are position and color.
    pub fn create_program_default(&mut self, key: impl Into<String>) {
        let program = Program::__new_default().unwrap();
        self.insert(key, program);
    }

    /// Create a new vertex array in the cache from the given vertex layout.
    pub fn create_vertex_array_from_vertex_layout(&mut self, key: impl Into<String>, vertex_layout: VertexLayout) {
        let vertex_array = InputLayout::__from_vertex_layout(vertex_layout);
        self.insert(key, vertex_array);
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
        map.remove(key).and_then(|v| v.downcast().ok())
            .map(|v| *v)
    }
}