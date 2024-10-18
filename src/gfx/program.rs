use std::ffi::CString;

use anyhow::Result;
use ggmath::prelude::*;

use crate::app::app_prelude::ShaderParameters;

use super::{
    input_parameters::RenderParameters,
    shader::Shader,
    shader_gen::{shader_parameters::SHADER_UNIFORM_PREFIX, shader_type::ShaderType},
    texture::TextureView,
};

/// Represents a GL program
pub struct Program {
    handle: u32,
    parameters: ShaderParameters,
}

impl !Send for Program {}
impl !Sync for Program {}

impl Program {
    /// Creates a new program
    /// # Safety
    /// This function is unsafe because it should only be used on the main thread.
    pub(crate) unsafe fn __new(shaders: &[Shader]) -> Result<Self> {
        // Create program
        let handle = unsafe { gl::CreateProgram() };

        // Attach shaders
        for shader in shaders {
            unsafe {
                gl::AttachShader(handle, shader.handle());
            }
        }

        // Link program
        unsafe {
            gl::LinkProgram(handle);
        }

        // Check for errors
        let mut success = 1;
        unsafe {
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut success);
        }

        // Return error if program failed to link
        if success == 0 {
            // Get error message length
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut len);
            }

            // Get error message
            let mut buffer = vec![0; len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    handle,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
            }

            // Bail with error message
            anyhow::bail!(String::from_utf8(buffer).unwrap());
        }

        // Detach shaders and combine parameters
        for shader in shaders {
            unsafe {
                gl::DetachShader(handle, shader.handle());
            }
        }

        // Combine parameters
        let parameters = shaders.iter().map(|shader| shader.parameters()).fold(
            ShaderParameters::new(),
            |mut acc, parameters| {
                acc.append(parameters).unwrap();
                acc
            },
        );

        Ok(Self { handle, parameters })
    }

    /// Get the GL handle
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Get the location of a uniform
    pub(crate) fn get_uniform_location(&self, name: &str) -> i32 {
        let name_cstring = CString::new(format!("{}{}", SHADER_UNIFORM_PREFIX, name)).unwrap();
        unsafe { gl::GetUniformLocation(self.handle, name_cstring.as_ptr()) }
    }

    /// Set the value of a uniform
    pub(crate) fn set_uniform(&self, name: &str, value: &dyn UniformValue) -> Result<()> {
        // Get the uniform location
        let location = self.get_uniform_location(name);

        // Return error if the uniform is not found
        if location == -1 {
            anyhow::bail!("Parameter {} not found in program", name);
        }

        // Set the uniform
        unsafe { value.set_uniform(location) };

        Ok(())
    }

    /// Get the parameters
    pub fn parameters(&self) -> &ShaderParameters {
        &self.parameters
    }

    /// Use the given input parameters
    pub fn use_parameters(&self, input_parameters: &RenderParameters) -> Result<()> {
        let expected_parameters = self.parameters();
        // Loop through the expected parameters
        for parameter in expected_parameters.iter() {
            // Get the corresponding input parameter value
            let value = input_parameters
                .get(parameter.name())
                .ok_or_else(|| anyhow::anyhow!("Expected input parameter {}", parameter.name()))?;

            // Verify that the types match
            if value.value_type() != parameter.value_type() {
                anyhow::bail!(
                    "Expected input parameter {} to be of type {:?}, but got {:?}",
                    parameter.name(),
                    parameter.value_type(),
                    value.value_type()
                );
            }

            // Set the uniform
            self.set_uniform(parameter.name(), value)?;
        }

        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

/// Represents a value that can be set as a uniform
pub trait UniformValue {
    /// Set the uniform
    /// # Safety
    /// This function is unsafe because it must be called on the main thread.
    /// It is also unsafe because it uses raw OpenGL functions.
    unsafe fn set_uniform(&self, location: i32);
    /// Get the `ShaderType` of the uniform
    fn value_type(&self) -> ShaderType;
}

impl UniformValue for f32 {
    unsafe fn set_uniform(&self, location: i32) {
        gl::Uniform1f(location, *self);
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::F32
    }
}

impl UniformValue for Vector2<f32> {
    unsafe fn set_uniform(&self, location: i32) {
        gl::Uniform2f(location, self.x(), self.y());
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Vec2
    }
}

impl UniformValue for Vector3<f32> {
    unsafe fn set_uniform(&self, location: i32) {
        gl::Uniform3f(location, self.x(), self.y(), self.z());
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Vec3
    }
}

impl UniformValue for Vector4<f32> {
    unsafe fn set_uniform(&self, location: i32) {
        gl::Uniform4f(location, self.x(), self.y(), self.z(), self.w());
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Vec4
    }
}

impl UniformValue for Matrix4x4<f32> {
    unsafe fn set_uniform(&self, location: i32) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr());
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Mat4
    }
}

impl UniformValue for TextureView {
    unsafe fn set_uniform(&self, location: i32) {
        // Get the appropriate texture unit
        let texture_unit = self.texture_type().texture_unit_index();

        // Bind the texture to the texture unit
        gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
        gl::BindTexture(gl::TEXTURE_2D, self.handle());

        // Set the uniform to the texture unit
        gl::Uniform1i(location, texture_unit as i32);
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Sampler2D
    }
}

/// Represents a value that can be set as a uniform with a default value
pub trait UniformDefault {
    fn default_value() -> Self;
}

impl UniformDefault for f32 {
    fn default_value() -> Self {
        0.0
    }
}

impl UniformDefault for Vector2<f32> {
    fn default_value() -> Self {
        vector!(0.0, 0.0)
    }
}

impl UniformDefault for Vector3<f32> {
    fn default_value() -> Self {
        vector!(0.0, 0.0, 0.0)
    }
}

impl UniformDefault for Vector4<f32> {
    fn default_value() -> Self {
        vector!(0.0, 0.0, 0.0, 0.0)
    }
}

impl UniformDefault for Matrix4x4<f32> {
    fn default_value() -> Self {
        Matrix4x4::identity()
    }
}

impl UniformDefault for TextureView {
    fn default_value() -> Self {
        Self::default()
    }
}
