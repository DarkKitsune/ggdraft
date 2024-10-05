use std::ffi::CString;

use anyhow::Result;

use super::shader::Shader;

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

    /// Use this program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
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
