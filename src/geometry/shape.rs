use std::{rc::Rc, vec};

use anyhow::Result;
use ggmath::prelude::*;

use crate::{
    color,
    gfx::{
        gfx_cache::GfxCache,
        texture::TextureView,
        vertex_layout::VertexLayout,
        vertex_list::{VertexList, VertexListInput},
    },
};

use super::orientation::{HasOrientation, Orientation};

/// A list of vertices and indices that represent a shape in 3D space using triangles.
pub struct ShapeTriangles {
    /// The positions of the vertices.
    positions: Vec<Vector3<f32>>,
    /// The normals of the vertices.
    normals: Vec<Vector3<f32>>,
    /// The colors of the vertices.
    colors: Vec<Vector4<f32>>,
    /// The texture coordinates of the vertices.
    tex_coords: Vec<Vector2<f32>>,
    /// The indices of the vertices that make up the triangles.
    indices: Vec<u32>,
}

impl ShapeTriangles {
    /// Creates a new `ShapeTriangles` with the given vertices and indices.
    /// # Safety
    /// This function is unsafe because it does not check if the indices are valid.
    /// It also does not check other arguments.
    pub(crate) const unsafe fn new_unchecked(
        positions: Vec<Vector3<f32>>,
        normals: Vec<Vector3<f32>>,
        colors: Vec<Vector4<f32>>,
        tex_coords: Vec<Vector2<f32>>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            positions,
            normals,
            colors,
            tex_coords,
            indices,
        }
    }

    /// Creates a new `ShapeTriangles` with the given vertices and indices.
    pub fn new(
        positions: Vec<Vector3<f32>>,
        normals: Vec<Vector3<f32>>,
        colors: Vec<Vector4<f32>>,
        tex_coords: Vec<Vector2<f32>>,
        indices: Vec<u32>,
    ) -> Result<Self> {
        // Ensure that the attributes have the same length.
        if positions.len() != normals.len()
            || positions.len() != colors.len()
            || positions.len() != tex_coords.len()
        {
            anyhow::bail!("Atrribute vectors must have the same length.");
        }

        // Ensure that the number of indices is a multiple of 3.
        if indices.len() % 3 != 0 {
            anyhow::bail!("The number of indices must be a multiple of 3.");
        }

        #[cfg(debug_assertions)]
        {
            // Ensure that all indices are valid.
            for index in &indices {
                if *index as usize >= positions.len() {
                    anyhow::bail!("Index out of bounds: {}", index);
                }
            }
        }

        Ok(unsafe { Self::new_unchecked(positions, normals, colors, tex_coords, indices) })
    }

    /// Creates an empty `ShapeTriangles`.
    /// This is useful for appending shapes.
    pub const fn empty() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Appends the triangles from another shape to this shape.
    pub fn append(&mut self, other: &mut ShapeTriangles) {
        let offset = self.positions.len() as u32;
        self.positions.append(&mut other.positions);
        self.normals.append(&mut other.normals);
        self.colors.append(&mut other.colors);
        self.tex_coords.append(&mut other.tex_coords);
        self.indices
            .append(&mut other.indices.iter().map(|i| i + offset).collect());
    }

    /// Returns the positions of the vertices.
    pub fn positions(&self) -> &[Vector3<f32>] {
        &self.positions
    }

    /// Returns the normals of the vertices.
    pub fn normals(&self) -> &[Vector3<f32>] {
        &self.normals
    }

    /// Returns the colors of the vertices.
    pub fn colors(&self) -> &[Vector4<f32>] {
        &self.colors
    }

    /// Returns the indices of the vertices that make up the triangles.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Returns the number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Convert this `ShapeTriangles` into a `VertexList` using the given layout.
    pub(crate) fn into_vertex_list(self, layout: Rc<VertexLayout>) -> Result<VertexList> {
        VertexList::new(
            layout,
            &[
                VertexListInput::Position(&self.positions),
                VertexListInput::Normal(&self.normals),
                VertexListInput::Color(&self.colors),
                VertexListInput::TexCoord(&self.tex_coords),
            ],
            self.indices,
        )
    }
}

/// A trait for shapes that can be converted to a list of triangle vertices.
pub trait ShapeToTriangles {
    /// Converts the shape to a list of triangle vertices.
    fn to_triangles(&self, cache: &GfxCache) -> ShapeTriangles;
}

// Implement ShapeToTriangles for Vec<T> where T: ShapeToTriangles.
impl<T> ShapeToTriangles for Vec<T>
where
    T: ShapeToTriangles,
{
    fn to_triangles(&self, cache: &GfxCache) -> ShapeTriangles {
        let mut triangles = ShapeTriangles::empty();

        for shape in self {
            let mut shape_triangles = shape.to_triangles(cache);
            triangles.append(&mut shape_triangles);
        }

        triangles
    }
}

/// A rectangle shape in 3D space.
pub struct Rectangle {
    /// The orientation of the rectangle.
    pub orientation: Orientation,
    /// The color of the rectangle.
    pub color: Vector4<f32>,
    /// The texture coordinates at the negative x, negative y corner.
    pub tex_coord_min: Vector2<f32>,
    /// The texture coordinates at the positive x, positive y corner.
    pub tex_coord_max: Vector2<f32>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub const fn new(
        center: Vector3<f32>,
        size: Vector2<f32>,
        rotation: Quaternion<f32>,
        color: Vector4<f32>,
    ) -> Self {
        Self {
            orientation: Orientation::new(center, rotation, vector!(size.x(), size.y(), 1.0)),
            color,
            tex_coord_min: vector!(0.0; 2),
            tex_coord_max: vector!(1.0; 2),
        }
    }

    /// Creates a new rectangle using the given orientation.
    /// The size of the rectangle will be the orientation's scale.
    pub const fn from_orientation(orientation: Orientation, color: Vector4<f32>) -> Self {
        Self {
            orientation,
            color,
            tex_coord_min: vector!(0.0; 2),
            tex_coord_max: vector!(1.0; 2),
        }
    }

    /// Sets the center of the rectangle.
    pub const fn with_center(mut self, center: Vector3<f32>) -> Self {
        self.orientation.set_position(center);
        self
    }

    /// Sets the size of the rectangle.
    pub const fn with_size(mut self, size: Vector2<f32>) -> Self {
        self.orientation.set_scale(vector!(size.x(), size.y(), 1.0));
        self
    }

    /// Sets the rotation of the rectangle.
    pub const fn with_rotation(mut self, rotation: Quaternion<f32>) -> Self {
        self.orientation.set_rotation(rotation);
        self
    }

    /// Sets the rotation of the rectangle (Z up)
    pub fn with_rotation_z(mut self, radians: f32) -> Self {
        self.orientation
            .set_rotation(Quaternion::from_rotation_z(radians));
        self
    }

    /// Sets the orientation of the rectangle.
    pub const fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the color of the rectangle.
    pub const fn with_color(mut self, color: Vector4<f32>) -> Self {
        self.color = color;
        self
    }

    /// Sets the texture coordinates of the rectangle.
    pub const fn with_tex_coords(mut self, min: Vector2<f32>, max: Vector2<f32>) -> Self {
        self.tex_coord_min = min;
        self.tex_coord_max = max;
        self
    }

    /// Sets the texture coordinates of the rectangle using the given `TextureView`
    pub fn with_texture_view_coords(mut self, texture_view: &TextureView) -> Self {
        self.tex_coord_min = texture_view.min_tex_coord();
        self.tex_coord_max = texture_view.max_tex_coord();
        self
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::from_orientation(Orientation::default(), color::WHITE)
    }
}

impl HasOrientation for Rectangle {
    fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    fn orientation_mut(&mut self) -> &mut Orientation {
        &mut self.orientation
    }
}

impl ShapeToTriangles for Rectangle {
    fn to_triangles(&self, _cache: &GfxCache) -> ShapeTriangles {
        // Get the transform matrix.
        let matrix = self.get_transform();

        // Calculate the positions.
        let positions = vec![
            (matrix * vector!(-0.5, -0.5, 0.0, 1.0)).xyz(),
            (matrix * vector!(0.5, -0.5, 0.0, 1.0)).xyz(),
            (matrix * vector!(0.5, 0.5, 0.0, 1.0)).xyz(),
            (matrix * vector!(-0.5, 0.5, 0.0, 1.0)).xyz(),
        ];

        // Calculate the normals and colors.
        let normal = (matrix * vector!(0.0, 0.0, 1.0, 1.0)).xyz().normalized();
        let normals = vec![normal; 4];
        let colors = vec![self.color; 4];

        // Calculate the texture coordinates.
        let tex_coords = vec![
            self.tex_coord_min,
            vector!(self.tex_coord_max.x(), self.tex_coord_min.y()),
            self.tex_coord_max,
            vector!(self.tex_coord_min.x(), self.tex_coord_max.y()),
        ];

        // Create the indices.
        let indices = vec![0, 1, 2, 0, 2, 3];

        unsafe { ShapeTriangles::new_unchecked(positions, normals, colors, tex_coords, indices) }
    }
}
