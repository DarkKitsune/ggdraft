use multiverse_ecs::prelude::*;

/// Trait extending Universes with reference functionality.
pub trait UniverseRefExt {
    /// Get a reference to the component pointed to by a `ComponentRef`.
    fn get_component<T: 'static>(&self, component_ref: &ComponentRef<T>) -> Option<&T>;

    /// Get a mutable reference to the component pointed to by a `ComponentRef`.
    fn get_component_mut<T: 'static>(&mut self, component_ref: &ComponentRef<T>) -> Option<&mut T>;

    /// Get a reference to the class object pointed to by a `ClassRef`.
    fn get_class<T: Class>(&self, class_ref: &ClassRef<T>) -> Option<&T>;

    /// Get a mutable reference to the class object pointed to by a `ClassRef`.
    fn get_class_mut<T: Class>(&mut self, class_ref: &ClassRef<T>) -> Option<&mut T>;
}

impl UniverseRefExt for Universe {
    fn get_component<T: 'static>(&self, component_ref: &ComponentRef<T>) -> Option<&T> {
        self.node(component_ref.node_handle())
            .and_then(|node| node.component::<T>())
    }

    fn get_component_mut<T: 'static>(&mut self, component_ref: &ComponentRef<T>) -> Option<&mut T> {
        self.node_mut(component_ref.node_handle())
            .and_then(|node| node.component_mut::<T>())
    }

    fn get_class<T: Class>(&self, class_ref: &ClassRef<T>) -> Option<&T> {
        self.node(class_ref.node_handle())
            .and_then(|node| node.class_as::<T>())
    }

    fn get_class_mut<T: Class>(&mut self, class_ref: &ClassRef<T>) -> Option<&mut T> {
        self.node_mut(class_ref.node_handle())
            .and_then(|node| node.class_as_mut::<T>())
    }
}

/// Reference to a node's component.
#[derive(Clone)]
pub struct ComponentRef<T> {
    /// Handle to the node whom owns the component.
    handle: NodeHandle,
    /// Phantom data to hold the component type.
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ComponentRef<T> {
    /// Get a new `ComponentRef` pointing to a component in the given node.
    pub fn of(handle: impl Into<NodeHandle>) -> Self {
        Self {
            handle: handle.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the handle to the node whom owns the component.
    pub fn node_handle(&self) -> &NodeHandle {
        &self.handle
    }
}

/// Reference to a node's class object.
#[derive(Clone)]
pub struct ClassRef<T: Class> {
    /// Handle to the node whom owns the class object.
    handle: NodeHandle,
    /// Phantom data to hold the class type.
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Class> ClassRef<T> {
    /// Get a new `ClassRef` pointing to a class object belonging to a node.
    pub fn of(handle: impl Into<NodeHandle>) -> Self {
        Self {
            handle: handle.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the handle to the node whom owns the class object.
    pub fn node_handle(&self) -> &NodeHandle {
        &self.handle
    }
}