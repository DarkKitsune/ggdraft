use std::{any::Any, ffi::CString};

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

    /// Set the value of a uniform
    pub(crate) fn set_uniform(&self, name: &str, value: &dyn UniformValue) -> Result<()> {
        // Set the uniform
        unsafe { value.set_uniform(self.handle, name) }
    }

    /// Get the parameters
    pub fn parameters(&self) -> &ShaderParameters {
        &self.parameters
    }

    /// Use the given input parameters
    pub(crate) fn use_parameters(&self, input_parameters: &RenderParameters) -> Result<()> {
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

/// Get the location of a uniform in the given program.
/// # Safety
/// This function is unsafe because it must be called on the main thread.
/// It is also unsafe because it uses raw OpenGL functions.
unsafe fn get_uniform_location(program: u32, name: &str) -> Result<i32> {
    let name_cstring = CString::new(format!("{}{}", SHADER_UNIFORM_PREFIX, name)).unwrap();
    let location = unsafe { gl::GetUniformLocation(program, name_cstring.as_ptr()) };
    if location == -1 {
        Err(anyhow::anyhow!(
            "Uniform {:?} not found in program",
            name_cstring
        ))
    } else {
        Ok(location)
    }
}

/// Represents a value that can be set as a uniform
pub trait UniformValue: Any {
    /// Copy this value to the uniform at the given location.
    /// # Safety
    /// This function is unsafe because it must be called on the main thread.
    /// It is also unsafe because it uses raw OpenGL functions.
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()>;
    /// Get the `ShaderType` of the uniform
    fn value_type(&self) -> ShaderType;
    /// Get the value as an `Any` trait object
    fn as_any(&self) -> &dyn Any;
}

impl UniformValue for f32 {
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()> {
        let location = get_uniform_location(program, name)?;

        gl::Uniform1f(location, *self);

        Ok(())
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::F32
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UniformValue for Vector2<f32> {
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()> {
        let location = get_uniform_location(program, name)?;

        gl::Uniform2f(location, self.x(), self.y());

        Ok(())
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Vec2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UniformValue for Vector3<f32> {
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()> {
        let location = get_uniform_location(program, name)?;

        gl::Uniform3f(location, self.x(), self.y(), self.z());

        Ok(())
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Vec3
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UniformValue for Vector4<f32> {
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()> {
        let location = get_uniform_location(program, name)?;

        gl::Uniform4f(location, self.x(), self.y(), self.z(), self.w());

        Ok(())
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Vec4
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UniformValue for Matrix4x4<f32> {
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()> {
        let location = get_uniform_location(program, name)?;

        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr());

        Ok(())
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Mat4
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UniformValue for TextureView {
    unsafe fn set_uniform(&self, program: u32, name: &str) -> Result<()> {
        let texture_location = get_uniform_location(program, name)?;
        let min_location = get_uniform_location(program, &format!("{}_min", name));
        let max_location = get_uniform_location(program, &format!("{}_max", name));

        // Get the appropriate texture unit
        let texture_unit = self.texture_type().texture_unit_index();

        // Bind the texture to the texture unit
        gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
        gl::BindTexture(gl::TEXTURE_2D, self.handle());

        // Set the texture uniform
        gl::Uniform1i(texture_location, texture_unit as i32);

        // Set the min and max uniforms (if they exist)
        if let Ok(min_location) = min_location {
            gl::Uniform3f(min_location, self.min().x(), self.min().y(), self.min().z());
        }
        if let Ok(max_location) = max_location {
            gl::Uniform3f(max_location, self.max().x(), self.max().y(), self.max().z());
        }

        Ok(())
    }

    fn value_type(&self) -> ShaderType {
        ShaderType::Sampler2D
    }

    fn as_any(&self) -> &dyn Any {
        self
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
