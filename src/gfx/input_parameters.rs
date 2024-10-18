use super::program::UniformValue;

/// Represents a render parameter for the render pipeline.
pub struct RenderParameter {
    name: String,
    value: Box<dyn UniformValue>,
}

/// Parameters for the render pipeline.
pub struct RenderParameters {
    parameters: Vec<RenderParameter>,
}

impl RenderParameters {
    /// Create a new set of render parameters.
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }

    /// Set the render parameter by name.
    /// This will overwrite any existing parameter with the same name.
    pub fn set<T: UniformValue + 'static>(&mut self, name: impl Into<String>, value: T) {
        let name = name.into();
        let value = Box::new(value);
        self.parameters.retain(|p| p.name != name);
        self.parameters.push(RenderParameter { name, value });
    }

    /// Get the render parameter by name.
    pub fn get(&self, name: &str) -> Option<&dyn UniformValue> {
        self.parameters
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.as_ref())
    }
}
