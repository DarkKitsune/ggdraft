use std::ffi::CString;

use anyhow::Result;

use super::shader::{Shader, ShaderStage};

/// Default shader source for a vertex shader
const DEFAULT_VERTEX_SHADER: &str = r#"
#version 450 core

layout(location = 0) in vec3 input_position;
layout(location = 1) in vec4 input_color;

layout(location = 0) out vec4 output_color;

out gl_PerVertex {
    vec4 gl_Position;
    float gl_PointSize;
};

void main() {
    gl_Position = vec4(input_position, 1.0);
    output_color = input_color;
}
"#;

/// Default shader source for a fragment shader
const DEFAULT_FRAGMENT_SHADER: &str = r#"
#version 450 core

layout(location = 0) in vec4 input_color;

out vec4 output_color;

void main() {
    output_color = input_color;
}
"#;

/// Represents a GL program
pub struct Program {
    handle: u32,
}

impl !Send for Program {}
impl !Sync for Program {}

impl Program {
    /// Creates a new program
    pub(crate) fn __new(shaders: &[Shader]) -> Result<Self> {
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

        // Detach shaders
        for shader in shaders {
            unsafe {
                gl::DetachShader(handle, shader.handle());
            }
        }

        Ok(Self { handle })
    }

    /// Create a new program with default shaders.
    /// The only vertex inputs are position and color.
    pub(crate) fn __new_default() -> Result<Self> {
        let vertex_shader = Shader::__new(DEFAULT_VERTEX_SHADER, ShaderStage::Vertex)?;
        let fragment_shader = Shader::__new(DEFAULT_FRAGMENT_SHADER, ShaderStage::Fragment)?;
        Self::__new(&[vertex_shader, fragment_shader])
    }

    /// Get the GL handle
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Get the location of a uniform
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let name_cstring = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.handle, name_cstring.as_ptr()) }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
