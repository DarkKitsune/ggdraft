use anyhow::Result;
use ggmath::prelude::Vector4;

use super::{shader_expression::ShaderExpression, shader_type::ShaderType};

pub(crate) const SHADER_OUTPUT_PREFIX: &str = "output_";

/// Represents a single output from a shader stage.
pub struct ShaderOutput {
    name: String,
    value_type: ShaderType,
    location: usize,
    expression: Option<ShaderExpression>,
}

impl ShaderOutput {
    pub(crate) fn new(name: &str, value_type: ShaderType, location: usize) -> Self {
        Self {
            name: name.to_string(),
            value_type,
            location,
            expression: None,
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

    pub fn expression(&self) -> Option<&ShaderExpression> {
        self.expression.as_ref()
    }
}

/// The outputs for a shader stage during shader generation.
/// Call `ShaderOutputs::set` to set the expression for an output.
pub struct ShaderOutputs {
    outputs: Vec<ShaderOutput>,
    is_vertex: bool,
    vertex_position: Option<ShaderExpression>
}

impl ShaderOutputs {
    pub(crate) fn new(is_vertex: bool) -> Self {
        Self {
            outputs: Vec::new(),
            is_vertex,
            vertex_position: None,
        }
    }

    pub fn output(&self, name: &str) -> Option<&ShaderOutput> {
        self.outputs.iter().find(|output| output.name() == name)
    }

    fn output_mut(&mut self, name: &str) -> Option<&mut ShaderOutput> {
        self.outputs.iter_mut().find(|output| output.name() == name)
    }

    /// Set the expression for the output with the given name.
    /// If the output does not exist, it will be created.
    pub fn set(&mut self, name: &str, expression: impl Into<ShaderExpression>) -> Result<()> {
        let expression = expression.into();
        if let Some(output) = self.output_mut(name) {
            output.expression = Some(expression);
        } else {
            let output = ShaderOutput::new(name, expression.shader_type()?, self.outputs.len());
            self.outputs.push(output);
            self.set(name, expression)?;
        }
        Ok(())
    }

    /// Set the expression for the vertex position output.
    /// Returns an error if this is not a vertex shader.
    pub fn set_vertex_position(&mut self, expression: ShaderExpression) -> Result<()> {
        if self.is_vertex {
            self.vertex_position = Some(expression);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Cannot set vertex position outside of a vertex shader"))
        }
    }

    /// Get the expression for the vertex position output.
    pub fn vertex_position(&self) -> Option<&ShaderExpression> {
        self.vertex_position.as_ref()
    }

    /// Iterate over the outputs.
    pub fn iter(&self) -> impl Iterator<Item = &ShaderOutput> {
        self.outputs.iter()
    }
}