use std::{any::{Any, TypeId}, collections::HashMap};

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