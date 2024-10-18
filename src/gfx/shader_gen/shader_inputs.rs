use anyhow::Result;

use super::{
    shader_expression::{ShaderExpression, ShaderOperation},
    shader_type::ShaderType,
};

/// The prefix for shader input variables.
pub(crate) const SHADER_INPUT_PREFIX: &str = "input_";

/// Represents a single input to a shader stage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderInput {
    name: String,
    value_type: ShaderType,
    location: usize,
}

impl ShaderInput {
    pub(crate) fn new(name: &str, value_type: ShaderType, location: usize) -> Self {
        Self {
            name: name.to_string(),
            value_type,
            location,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value_type(&self) -> &ShaderType {
        &self.value_type
    }

    pub fn location(&self) -> usize {
        self.location
    }

    pub fn to_expression(&self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Input(self.name.clone(), self.value_type))
    }
}

/// The inputs for a shader stage during shader generation.
pub struct ShaderInputs {
    inputs: Vec<ShaderInput>,
}

impl ShaderInputs {
    /// Create a new set of shader inputs.
    /// Returns an error if the inputs are empty or if there are duplicate names.
    pub(crate) fn with_inputs(inputs: Vec<ShaderInput>) -> Result<Self> {
        // Check for duplicate names
        let mut names = std::collections::HashSet::new();
        for input in &inputs {
            if !names.insert(input.name()) {
                return Err(anyhow::anyhow!("Duplicate input: {}", input.name()));
            }
        }
        // Check for empty inputs
        if inputs.is_empty() {
            return Err(anyhow::anyhow!("No inputs provided"));
        }
        Ok(Self { inputs })
    }

    /// Get the input with the given name.
    pub fn input(&self, name: impl AsRef<str>) -> Option<&ShaderInput> {
        self.inputs
            .iter()
            .find(|input| input.name() == name.as_ref())
    }

    /// Get the input with the given name as a shader expression.
    pub fn get(&self, name: impl AsRef<str>) -> Result<ShaderExpression> {
        let name = name.as_ref();
        self.input(name)
            .map(|input| input.to_expression())
            .ok_or_else(|| anyhow::anyhow!("Input not found: {}", name))
    }

    /// Get an iterator over the inputs.
    pub fn iter(&self) -> impl Iterator<Item = &ShaderInput> {
        self.inputs.iter()
    }
}
