use anyhow::Result;

use super::{
    buffer::VertexBuffer,
    shader::ShaderStage,
    shader_gen::{
        shader_inputs::{ShaderInput, ShaderInputs, SHADER_INPUT_PREFIX},
        shader_outputs::{ShaderOutputs, SHADER_OUTPUT_PREFIX},
    },
    vertex_layout::VertexLayout,
};

// The location the vertex buffer should be bound to.
pub(crate) const VERTEX_BUFFER_LOCATION: u32 = 0;
// The location the instance buffer should be bound to.
pub(crate) const _INSTANCE_BUFFER_LOCATION: u32 = 1;

/// Layout describing a set of vertex and instance inputs for rendering.
pub struct InputLayout {
    layout: VertexLayout,
    handle: u32,
}

impl !Send for InputLayout {}
impl !Sync for InputLayout {}

impl InputLayout {
    /// Create a new vertex array from the given vertex layout.
    // TODO: Add instancing support.
    pub(crate) fn __from_vertex_layout(layout: VertexLayout) -> Self {
        let mut handle = 0;

        unsafe {
            // Create a vertex array
            gl::CreateVertexArrays(1, &mut handle);

            // Enable the vertex attributes
            let mut offset = 0;
            for (index, input) in layout.inputs().iter().enumerate() {
                gl::EnableVertexArrayAttrib(handle, index as u32);
                gl::VertexArrayAttribFormat(
                    handle,
                    index as u32,
                    input.component_count() as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    offset as u32,
                );
                gl::VertexArrayAttribBinding(handle, index as u32, VERTEX_BUFFER_LOCATION);
                gl::VertexArrayBindingDivisor(handle, index as u32, 0);
                offset += input.component_count() * std::mem::size_of::<f32>();
            }
        }

        Self { layout, handle }
    }

    /// Get the GL handle.
    pub fn vertex_array_handle(&self) -> u32 {
        self.handle
    }

    /// Get the vertex layout of the buffer.
    pub fn layout(&self) -> &VertexLayout {
        &self.layout
    }

    /// Get the vertex stride.
    pub fn byte_stride(&self) -> usize {
        self.layout.byte_stride()
    }

    /// Validate a vertex buffer for this input layout.
    /// Returns an error if the buffer is not compatible with the layout.
    pub fn validate_buffer(&self, buffer: &VertexBuffer) -> Result<()> {
        if buffer.vertex_layout() != Some(&self.layout) {
            anyhow::bail!("Buffer is not compatible with input layout.");
        }
        Ok(())
    }

    /// Generate GLSL vertex and fragment shader code for the input layout.
    pub(crate) fn generate_vertex_fragment_shaders(
        &self,
        vertex: impl FnOnce(&ShaderInputs, &mut ShaderOutputs) -> Result<()>,
        fragment: impl FnOnce(&ShaderInputs, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<(String, String)> {
        // Create the vertex shader and fragment shader inputs.
        let (vertex_shader, fragment_inputs) = self
            .__generate_vertex_shader(vertex)
            .map_err(|e| anyhow::anyhow!("Failed to generate vertex shader: {}", e))?;
        let fragment_shader = self
            .__generate_fragment_shader(fragment_inputs, fragment)
            .map_err(|e| anyhow::anyhow!("Failed to generate fragment shader: {}", e))?;
        Ok((vertex_shader, fragment_shader))
    }

    /// Generate a GLSL vertex shader for the input layout.
    /// Also returns the inputs for the corresponding fragment shader.
    pub(crate) fn __generate_vertex_shader(
        &self,
        f: impl FnOnce(&ShaderInputs, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<(String, ShaderInputs)> {
        // Create the shader inputs from the vertex layout's inputs.
        let mut location = 0;
        let inputs = ShaderInputs::with_inputs(
            self.layout
                .inputs()
                .iter()
                .map(|input| {
                    // Get the shader type for the input.
                    let shader_type = input.shader_type();

                    // Create a new shader input.
                    let input = ShaderInput::new(input.name(), shader_type, location);

                    // Increment the binding location.
                    location += shader_type.location_count();

                    input
                })
                .collect(),
        );

        // Create the shader outputs.
        let mut outputs = ShaderOutputs::new(ShaderStage::Vertex);

        // Call the closure to generate the shader code.
        f(&inputs, &mut outputs)?;

        // Generate the shader code.
        let mut code = "#version 450\n".to_string();

        // Add the inputs.
        for input in inputs.iter() {
            code += &format!(
                "layout(location = {}) in {} {}{};\n",
                input.location(),
                input.value_type().glsl_name(),
                SHADER_INPUT_PREFIX,
                input.name()
            );
        }

        // Add the outputs.
        for output in outputs.iter() {
            code += &format!(
                "layout(location = {}) out {} {}{};\n",
                output.location(),
                output.value_type().glsl_name(),
                SHADER_OUTPUT_PREFIX,
                output.name()
            );
        }

        // Add the gl_PerVertex block.
        code += "out gl_PerVertex {\n";
        code += "vec4 gl_Position;\n";
        code += "};\n";

        // Begin the main function.
        code += "void main() {\n";

        // Set the vertex position.
        code += &format!(
            "gl_Position = {};\n",
            outputs
                .vertex_position()
                .ok_or_else(|| anyhow::anyhow!("Vertex position not set."))?
        );

        // Set the other outputs.
        for output in outputs.iter() {
            if let Some(expression) = output.expression() {
                code += &format!(
                    "{}{} = {};\n",
                    SHADER_OUTPUT_PREFIX,
                    output.name(),
                    expression
                );
            }
        }

        // End the main function.
        code += "}\n";

        // Build the fragment shader inputs.
        let fragment_inputs = ShaderInputs::with_inputs(
            outputs
                .iter()
                .map(|output| {
                    ShaderInput::new(
                        output.name(),
                        output.value_type().clone(),
                        output.location(),
                    )
                })
                .collect(),
        );

        Ok((code, fragment_inputs))
    }

    /// Generate a GLSL fragment shader for the input layout.
    /// The fragment shader inputs are provided as an argument.
    pub(crate) fn __generate_fragment_shader(
        &self,
        inputs: ShaderInputs,
        f: impl FnOnce(&ShaderInputs, &mut ShaderOutputs) -> Result<()>,
    ) -> Result<String> {
        // Create the shader outputs.
        let mut outputs = ShaderOutputs::new(ShaderStage::Fragment);

        // Call the closure to generate the shader code.
        f(&inputs, &mut outputs)?;

        // Generate the shader code.
        let mut code = "#version 450\n".to_string();

        // Add the inputs.
        for input in inputs.iter() {
            code += &format!(
                "layout(location = {}) in {} {}{};\n",
                input.location(),
                input.value_type().glsl_name(),
                SHADER_INPUT_PREFIX,
                input.name()
            );
        }

        // Add the fragment color output.
        code += "layout(location = 0) out vec4 out_fragment_color;\n";

        // Add the outputs.
        for output in outputs.iter() {
            code += &format!(
                "layout(location = {}) out {} {}{};\n",
                output.location(),
                output.value_type().glsl_name(),
                SHADER_OUTPUT_PREFIX,
                output.name()
            );
        }

        // Begin the main function.
        code += "void main() {\n";

        // Set the fragment color.
        code += &format!(
            "out_fragment_color = {};\n",
            outputs
                .fragment_color()
                .ok_or_else(|| anyhow::anyhow!("Fragment color not set."))?
        );

        // Set the other outputs.
        for output in outputs.iter() {
            if let Some(expression) = output.expression() {
                code += &format!(
                    "{}{} = {};\n",
                    SHADER_OUTPUT_PREFIX,
                    output.name(),
                    expression
                );
            }
        }

        // End the main function.
        code += "}\n";

        Ok(code)
    }
}

impl Drop for InputLayout {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.handle);
        }
    }
}
