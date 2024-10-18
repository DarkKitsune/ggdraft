use std::ffi::CString;

use anyhow::Result;

use crate::app::app_prelude::ShaderParameters;

/// Represents a GL shader
pub struct Shader {
    handle: u32,
    stage: ShaderStage,
    parameters: ShaderParameters,
}

impl !Send for Shader {}
impl !Sync for Shader {}

impl Shader {
    /// Creates a new shader
    /// # Safety
    /// This function is unsafe because it should only be used on the main thread.
    pub(crate) unsafe fn __new(
        stage: ShaderStage,
        source: &str,
        parameters: ShaderParameters,
    ) -> Result<Self> {
        // Create shader
        let handle = unsafe { gl::CreateShader(stage.to_gl_enum()) };

        // Set shader source
        let source_cstring = CString::new(source).unwrap();
        unsafe {
            gl::ShaderSource(handle, 1, &source_cstring.as_ptr(), std::ptr::null());
            gl::CompileShader(handle);
        }

        // Check for errors
        let mut success = 1;
        unsafe {
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut success);
        }

        // Return error if shader failed to compile
        if success == 0 {
            // Get error message length
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
            }

            // Get error message
            let mut buffer = vec![0; len as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    handle,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
            }

            // Bail with error message
            anyhow::bail!(String::from_utf8(buffer).unwrap());
        }
        Ok(Self {
            handle,
            stage,
            parameters,
        })
    }

    /// Get the GL handle
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Get the shader stage
    pub fn stage(&self) -> ShaderStage {
        self.stage
    }

    /// Get the parameters
    pub fn parameters(&self) -> &ShaderParameters {
        &self.parameters
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}

/// Represents a shader stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    Vertex,
    Fragment,
}

impl ShaderStage {
    /// Convert to the corresponding GL enum
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            ShaderStage::Vertex => gl::VERTEX_SHADER,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}
