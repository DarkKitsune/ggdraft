use ggmath::prelude::*;

use super::{
    program::UniformValue,
    shader_gen::shader_parameters::{
        PARAMETER_MODEL_MATRIX, PARAMETER_PROJECTION_MATRIX, PARAMETER_VIEW_MATRIX,
    },
};

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

    /// Set the view matrix.
    /// The actual name of the parameter is `ShaderParameter::PARAMETER_VIEW_MATRIX`.
    pub fn set_view_matrix(&mut self, matrix: Matrix4x4<f32>) {
        self.set(PARAMETER_VIEW_MATRIX, matrix);
    }

    /// Get the view matrix.
    pub fn get_view_matrix(&self) -> Option<&Matrix4x4<f32>> {
        self.get(PARAMETER_VIEW_MATRIX)
            .map(|v| v.as_any().downcast_ref().unwrap())
    }

    /// Set the projection matrix.
    /// The actual name of the parameter is `ShaderParameter::PARAMETER_PROJECTION_MATRIX`.
    pub fn set_projection_matrix(&mut self, matrix: Matrix4x4<f32>) {
        self.set(PARAMETER_PROJECTION_MATRIX, matrix);
    }

    /// Get the projection matrix.
    pub fn get_projection_matrix(&self) -> Option<&Matrix4x4<f32>> {
        self.get(PARAMETER_PROJECTION_MATRIX)
            .map(|v| v.as_any().downcast_ref().unwrap())
    }

    /// Set the model matrix.
    /// The actual name of the parameter is `ShaderParameter::PARAMETER_MODEL_MATRIX`.
    pub fn set_model_matrix(&mut self, matrix: Matrix4x4<f32>) {
        self.set(PARAMETER_MODEL_MATRIX, matrix);
    }

    /// Get the model matrix.
    pub fn get_model_matrix(&self) -> Option<&Matrix4x4<f32>> {
        self.get(PARAMETER_MODEL_MATRIX)
            .map(|v| v.as_any().downcast_ref().unwrap())
    }
}
