use anyhow::Result;

use crate::gfx::shader::ShaderStage;

use super::{shader_expression::ShaderExpression, shader_type::ShaderType};

/// The prefix for shader output variables.
pub(crate) const SHADER_OUTPUT_PREFIX: &str = "_output_";

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
    stage: ShaderStage,
    vertex_position: Option<ShaderExpression>,
    fragment_color: Option<ShaderExpression>,
}

impl ShaderOutputs {
    pub(crate) fn new(stage: ShaderStage) -> Self {
        Self {
            outputs: Vec::new(),
            stage,
            vertex_position: None,
            fragment_color: None,
        }
    }

    pub fn output(&self, name: impl AsRef<str>) -> Option<&ShaderOutput> {
        self.outputs
            .iter()
            .find(|output| output.name() == name.as_ref())
    }

    fn output_mut(&mut self, name: impl AsRef<str>) -> Option<&mut ShaderOutput> {
        self.outputs
            .iter_mut()
            .find(|output| output.name() == name.as_ref())
    }

    /// Set the expression for the output with the given name.
    /// If the output does not exist, it will be created.
    pub fn set(
        &mut self,
        name: impl AsRef<str>,
        expression: impl Into<ShaderExpression>,
    ) -> Result<()> {
        let name = name.as_ref();
        let expression = expression.into();
        if let Some(output) = self.output_mut(name) {
            output.expression = Some(expression);
        } else {
            // Location is the index of the output in the list.
            // If this is a fragment shader, then add 1 to the location to account for the color output.
            let location = self.outputs.len()
                + if self.stage == ShaderStage::Fragment {
                    1
                } else {
                    0
                };
            let output = ShaderOutput::new(name, expression.shader_type()?, location);
            self.outputs.push(output);
            self.set(name, expression)?;
        }
        Ok(())
    }

    /// Set the expression for the vertex position output.
    pub fn set_vertex_position(&mut self, expression: ShaderExpression) {
        // Panic if this is not a vertex shader.
        if self.stage != ShaderStage::Vertex {
            panic!("Cannot set vertex position in a non-vertex shader");
        }

        // Panic if the expression is not a vec4.
        let shader_type = expression.shader_type().unwrap();
        if shader_type != ShaderType::Vec4 {
            panic!(
                "Vertex position type must be {}, found {}",
                ShaderType::Vec4.rust_name(),
                shader_type.rust_name()
            );
        }

        self.vertex_position = Some(expression);
    }

    /// Set the expression for the fragment color output.
    pub fn set_fragment_color(&mut self, expression: ShaderExpression) {
        // Panic if this is not a fragment shader.
        if self.stage != ShaderStage::Fragment {
            panic!("Cannot set fragment color in a non-fragment shader");
        }

        // Panic if the expression is not a vec4.
        let shader_type = expression.shader_type().unwrap();
        if shader_type != ShaderType::Vec4 {
            panic!(
                "Fragment color type must be {}, found {}",
                ShaderType::Vec4.rust_name(),
                shader_type.rust_name()
            );
        }

        self.fragment_color = Some(expression);
    }

    /// Get the expression for the vertex position output.
    pub fn vertex_position(&self) -> Option<&ShaderExpression> {
        self.vertex_position.as_ref()
    }

    /// Get the expression for the fragment color output.
    pub fn fragment_color(&self) -> Option<&ShaderExpression> {
        self.fragment_color.as_ref()
    }

    /// Iterate over the outputs.
    pub fn iter(&self) -> impl Iterator<Item = &ShaderOutput> {
        self.outputs.iter()
    }
}
