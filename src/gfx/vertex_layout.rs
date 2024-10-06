use anyhow::Result;

// Allowed type for vertex data.
pub type VertexComponent = f32;

/// Represents a single vertex input.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VertexInput {
    Position,
    Normal,
    Color,
    TexCoord,
}

impl VertexInput {
    /// Get the # of components for this input.
    pub fn component_count(&self) -> usize {
        match self {
            VertexInput::Position => 3,
            VertexInput::Normal => 3,
            VertexInput::Color => 4,
            VertexInput::TexCoord => 2,
        }
    }

    /// Get the byte size of this input.
    pub fn byte_size(&self) -> usize {
        self.component_count() * std::mem::size_of::<VertexComponent>()
    }

    /// Get the name of this input for shader generation.
    /// Returns a `String` because this may support custom inputs in the future.
    pub fn shader_name(&self) -> String {
        match self {
            VertexInput::Position => "input_position".to_string(),
            VertexInput::Normal => "input_normal".to_string(),
            VertexInput::Color => "input_color".to_string(),
            VertexInput::TexCoord => "input_tex_coord".to_string(),
        }
    }
}

/// Represents the layout of a tightly-packed vertex in memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexLayout {
    inputs: Vec<VertexInput>,
    component_stride: usize,
}

impl VertexLayout {
    /// Create a new vertex layout.
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            component_stride: 0,
        }
    }

    /// Create a vertex layout from the given inputs.
    pub fn from_inputs(inputs: Vec<VertexInput>) -> Self {
        let mut layout = Self::new();
        layout.push_many(inputs);
        layout
    }

    /// Push a new input to the layout.
    pub fn push(&mut self, input: VertexInput) {
        self.component_stride += input.component_count();
        self.inputs.push(input);
    }

    /// Push multiple inputs to the layout.
    pub fn push_many(&mut self, inputs: Vec<VertexInput>) {
        self.component_stride += inputs
            .iter()
            .map(|input| input.component_count())
            .sum::<usize>();
        self.inputs.extend(inputs);
    }

    /// Push a new input to the layout.
    pub fn with_input(mut self, input: VertexInput) -> Self {
        self.push(input);
        self
    }

    /// Push multiple inputs to the layout.
    pub fn with_inputs(mut self, inputs: Vec<VertexInput>) -> Self {
        self.push_many(inputs);
        self
    }

    /// Get the inputs in the layout.
    pub fn inputs(&self) -> &[VertexInput] {
        &self.inputs
    }

    /// Validate the layout for correctness.
    pub fn validate(&self) -> Result<()> {
        // Check for duplicate inputs.
        let mut seen = Vec::new();
        for input in &self.inputs {
            if seen.contains(&input) {
                anyhow::bail!("Duplicate input found in vertex layout.");
            }
            seen.push(input);
        }
        Ok(())
    }

    /// Validate the given vertex data for this layout.
    pub fn validate_data(&self, data: &[VertexComponent]) -> Result<()> {
        // Check for correct data size.
        if data.len() % self.component_stride != 0 {
            anyhow::bail!("Vertex data was invalid for layout: wrong size.");
        }
        Ok(())
    }

    /// Get the component stride of this layout (the size of one vertex in components).
    pub fn component_stride(&self) -> usize {
        self.component_stride
    }

    /// Get the byte stride of this layout (the size of one vertex in bytes).
    pub fn byte_stride(&self) -> usize {
        self.component_stride * std::mem::size_of::<VertexComponent>()
    }
}

impl Default for VertexLayout {
    fn default() -> Self {
        Self::from_inputs(vec![
            VertexInput::Position,
            VertexInput::Normal,
            VertexInput::Color,
            VertexInput::TexCoord,
        ])
    }
}
