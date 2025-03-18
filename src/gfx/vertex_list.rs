use std::rc::Rc;

use anyhow::Result;
use ggmath::prelude::*;
use ggutil::prelude::MaybeOwned;

use crate::geometry::shape::ShapeToTriangles;

use super::{
    gfx_cache::GfxCache,
    vertex_layout::{VertexComponent, VertexInput, VertexLayout},
};

/// Represents an input for vertices going into a VertexList.
#[derive(Debug, Clone, PartialEq)]
pub enum VertexListInput<'a> {
    Position(&'a [Vector3<VertexComponent>]),
    Normal(&'a [Vector3<VertexComponent>]),
    Color(&'a [Vector4<VertexComponent>]),
    TexCoord(&'a [Vector2<VertexComponent>]),
}

impl<'a> VertexListInput<'a> {
    /// Get the input type.
    pub const fn input_type(&self) -> VertexInput {
        match self {
            VertexListInput::Position(_) => VertexInput::Position,
            VertexListInput::Normal(_) => VertexInput::Normal,
            VertexListInput::Color(_) => VertexInput::Color,
            VertexListInput::TexCoord(_) => VertexInput::TexCoord,
        }
    }

    /// Get the number of inputs.
    pub const fn len(&self) -> usize {
        match self {
            VertexListInput::Position(data) => data.len(),
            VertexListInput::Normal(data) => data.len(),
            VertexListInput::Color(data) => data.len(),
            VertexListInput::TexCoord(data) => data.len(),
        }
    }

    /// Copy the input data into the given buffer using the given stride.
    pub fn copy_to(&self, target: &mut [VertexComponent], component_stride: usize) {
        match self {
            VertexListInput::Position(data) => {
                for (i, v) in data.iter().enumerate() {
                    let offset = i * component_stride;

                    target[offset] = v.x();
                    target[offset + 1] = v.y();
                    target[offset + 2] = v.z();
                }
            }
            VertexListInput::Normal(data) => {
                for (i, v) in data.iter().enumerate() {
                    let offset = i * component_stride;
                    target[offset] = v.x();
                    target[offset + 1] = v.y();
                    target[offset + 2] = v.z();
                }
            }
            VertexListInput::Color(data) => {
                for (i, v) in data.iter().enumerate() {
                    let offset = i * component_stride;
                    target[offset] = v.x();
                    target[offset + 1] = v.y();
                    target[offset + 2] = v.z();
                    target[offset + 3] = v.w();
                }
            }
            VertexListInput::TexCoord(data) => {
                for (i, v) in data.iter().enumerate() {
                    let offset = i * component_stride;
                    target[offset] = v.x();
                    target[offset + 1] = v.y();
                }
            }
        }
    }
}

/// Represents a list of vertices.
pub struct VertexList {
    layout: Rc<VertexLayout>,
    data: Vec<VertexComponent>,
    indices: Vec<u32>,
}

impl VertexList {
    /// Create a new vertex list.
    pub fn new(
        layout: Rc<VertexLayout>,
        inputs: &[VertexListInput],
        indices: Vec<u32>,
    ) -> Result<Self> {
        // Ensure all inputs have the same length.
        let len = inputs[0].len();
        for input in inputs.iter().skip(1) {
            if input.len() != len {
                anyhow::bail!("All inputs must have the same length.");
            }
        }

        // Ensure that the inputs and indices are not empty.
        if len == 0 || indices.is_empty() {
            anyhow::bail!("Inputs and indices must not be empty.");
        }

        // Allocate the data buffer.
        let mut data = vec![0f32; layout.component_stride() * len];

        // Iterate over the layout's expected inputs and copy the data into the buffer.
        let mut data_offset = 0;
        for layout_input in layout.inputs() {
            // Find the matching provided input, or error if it wasn't provided.
            let matching_input = inputs
                .iter()
                .find(|list_input| list_input.input_type() == *layout_input)
                .ok_or_else(|| anyhow::anyhow!("Input type {:?} was not provided", layout_input))?;

            // Copy the input data into the buffer.
            matching_input.copy_to(&mut data[data_offset..], layout.component_stride());

            // Move the data offset.
            data_offset += layout_input.component_count();
        }

        Ok(Self {
            layout,
            data,
            indices,
        })
    }

    /// Create a new vertex list from the given shape.
    pub fn from_shape(
        cache: &GfxCache,
        layout: Rc<VertexLayout>,
        shape: &impl ShapeToTriangles,
    ) -> Result<Self> {
        shape.to_triangles(cache).into_vertex_list(layout)
    }

    /// Get the vertex data within the vertex list.
    pub fn vertex_data(&self) -> &[VertexComponent] {
        &self.data
    }

    /// Get the layout of the vertex list.
    pub fn layout(&self) -> &VertexLayout {
        &self.layout
    }

    /// Get the indices of the vertex list.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

/// Trait for types that can be converted into a `VertexList`.
pub trait IntoVertexList<'a> {
    /// Convert the type into a `VertexList`.
    fn into_vertex_list(
        self,
        cache: &GfxCache,
        layout: Rc<VertexLayout>,
    ) -> MaybeOwned<'a, VertexList>;
}

impl<'a> IntoVertexList<'a> for VertexList {
    fn into_vertex_list(
        self,
        _cache: &GfxCache,
        _layout: Rc<VertexLayout>,
    ) -> MaybeOwned<'a, VertexList> {
        MaybeOwned::Owned(self)
    }
}

impl<'a> IntoVertexList<'a> for &'a VertexList {
    fn into_vertex_list(
        self,
        _cache: &GfxCache,
        _layout: Rc<VertexLayout>,
    ) -> MaybeOwned<'a, VertexList> {
        MaybeOwned::Borrowed(self)
    }
}

impl<'a, T: ShapeToTriangles> IntoVertexList<'a> for &'a T {
    fn into_vertex_list(
        self,
        cache: &GfxCache,
        layout: Rc<VertexLayout>,
    ) -> MaybeOwned<'a, VertexList> {
        MaybeOwned::Owned(VertexList::from_shape(cache, layout, self).unwrap())
    }
}

impl<'a, T: ShapeToTriangles> IntoVertexList<'a> for Vec<T> {
    fn into_vertex_list(
        self,
        cache: &GfxCache,
        layout: Rc<VertexLayout>,
    ) -> MaybeOwned<'a, VertexList> {
        MaybeOwned::Owned(VertexList::from_shape(cache, layout, &self).unwrap())
    }
}
