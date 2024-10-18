use super::program::UniformValue;

/// Represents an input parameter for the render pipeline.
pub struct InputParameter {
    name: String,
    value: Box<dyn UniformValue>,
}

/// Input parameters for the render pipeline.
pub struct InputParameters {
    parameters: Vec<InputParameter>,
}

impl InputParameters {
    /// Create a new set of input parameters.
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }

    /// Set the input parameter by name.
    /// This will overwrite any existing parameter with the same name.
    pub fn set<T: UniformValue + 'static>(&mut self, name: impl Into<String>, value: T) {
        let name = name.into();
        let value = Box::new(value);
        self.parameters.retain(|p| p.name != name);
        self.parameters.push(InputParameter { name, value });
    }

    /// Get the input parameter by name.
    pub fn get(&self, name: &str) -> Option<&dyn UniformValue> {
        self.parameters
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.as_ref())
    }
}
