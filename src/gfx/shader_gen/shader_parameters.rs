use anyhow::Result;
use ggmath::prelude::*;

use crate::gfx::program::{UniformDefault, UniformValue};

use super::{
    prelude::{ShaderExpression, ShaderOperation},
    shader_type::ShaderType,
};

/// The prefix for shader uniform variables in generated shaders.
pub(crate) const SHADER_UNIFORM_PREFIX: &str = "_uniform_";

/// The built-in view matrix parameter name in generated shaders.
pub(crate) const PARAMETER_VIEW_MATRIX: &str = "builtin_view_matrix";

/// The built-in projection matrix parameter name in generated shaders.
pub(crate) const PARAMETER_PROJECTION_MATRIX: &str = "builtin_projection_matrix";

/// The built-in model matrix parameter name in generated shaders.
pub(crate) const PARAMETER_MODEL_MATRIX: &str = "builtin_model_matrix";

/// Represents a single parameter for a shader.
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderParameter {
    name: String,
    value_type: ShaderType,
}

impl ShaderParameter {
    /// Create a new shader parameter.
    pub(crate) fn new(name: impl Into<String>, value_type: ShaderType) -> Self {
        Self {
            name: name.into(),
            value_type,
        }
    }

    /// Get the name of the parameter.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the type of the parameter.
    pub fn value_type(&self) -> ShaderType {
        self.value_type
    }

    /// Get an expression pointing to this parameter.
    pub fn to_expression(&self) -> ShaderExpression {
        ShaderExpression::new(ShaderOperation::Uniform(self.name.clone(), self.value_type))
    }
}

/// The parameters for a shader during generation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderParameters {
    parameters: Vec<ShaderParameter>,
}

impl ShaderParameters {
    /// Create a new set of shader parameters.
    pub(crate) fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }

    /// Get the given parameter by name.
    /// Returns None if the parameter does not exist.
    pub fn parameter(&self, name: impl AsRef<str>) -> Option<&ShaderParameter> {
        let name = name.as_ref();
        self.parameters.iter().find(|p| p.name() == name)
    }

    /// Get the given parameter by name as an expression.
    /// This function will create the parameter if it does not exist.
    /// Panics if the parameter already exists with a different type.
    pub fn get<T: UniformValue + UniformDefault>(
        &mut self,
        name: impl Into<String>,
    ) -> ShaderExpression {
        let name = name.into();

        // Get the type for T.
        let value_type = T::value_type(&T::default_value());

        // Check if the parameter already exists.
        if let Some(parameter) = self.parameter(&name) {
            // If it does exist, first verify that the types match.
            if parameter.value_type() != value_type {
                panic!(
                    "Parameter {} was previously requested with type {:?}, but now requested with type {:?}",
                    name,
                    parameter.value_type(),
                    value_type
                );
            }

            // Return an expression pointing to the parameter.
            parameter.to_expression()
        } else {
            // If it does not exist, create the parameter.
            let parameter = ShaderParameter::new(&name, value_type);
            self.parameters.push(parameter);

            // Return an expression pointing to the parameter.
            self.parameter(&name).unwrap().to_expression()
        }
    }

    /// Get the given f32 parameter by name.
    pub fn get_f32(&mut self, name: impl Into<String>) -> ShaderExpression {
        self.get::<f32>(name)
    }

    /// Get the given Vector2<f32> parameter by name.
    pub fn get_vec2(&mut self, name: impl Into<String>) -> ShaderExpression {
        self.get::<Vector2<f32>>(name)
    }

    /// Get the given Vector3<f32> parameter by name.
    pub fn get_vec3(&mut self, name: impl Into<String>) -> ShaderExpression {
        self.get::<Vector3<f32>>(name)
    }

    /// Get the given Vector4<f32> parameter by name.
    pub fn get_vec4(&mut self, name: impl Into<String>) -> ShaderExpression {
        self.get::<Vector4<f32>>(name)
    }

    /// Get the given Matrix4x4<f32> parameter by name.
    pub fn get_mat3(&mut self, name: impl Into<String>) -> ShaderExpression {
        self.get::<Matrix4x4<f32>>(name)
    }

    /// Get the view matrix.
    pub fn get_view_matrix(&mut self) -> ShaderExpression {
        self.get::<Matrix4x4<f32>>(PARAMETER_VIEW_MATRIX)
    }

    /// Get the projection matrix.
    pub fn get_projection_matrix(&mut self) -> ShaderExpression {
        self.get::<Matrix4x4<f32>>(PARAMETER_PROJECTION_MATRIX)
    }

    /// Get the model matrix.
    pub fn get_model_matrix(&mut self) -> ShaderExpression {
        self.get::<Matrix4x4<f32>>(PARAMETER_MODEL_MATRIX)
    }

    /// Get an iterator over the parameters.
    pub fn iter(&self) -> impl Iterator<Item = &ShaderParameter> {
        self.parameters.iter()
    }

    /// Append the parameters from the given set of parameters.
    /// Returns an error if there are duplicate parameters with different types.
    pub fn append(&mut self, other: &Self) -> Result<()> {
        for parameter in other.iter() {
            // Check if the parameter already exists.
            if let Some(existing) = self.parameter(parameter.name()) {
                // If it does exist, verify that the types match.
                if existing.value_type() != parameter.value_type() {
                    anyhow::bail!(
                        "Parameter {} was previously requested with type {:?}, but now requested with type {:?}",
                        parameter.name(),
                        existing.value_type(),
                        parameter.value_type()
                    );
                }
            } else {
                // If it does not exist, add the parameter.
                self.parameters.push(parameter.clone());
            }
        }
        Ok(())
    }
}
