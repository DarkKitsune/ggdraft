use anyhow::Result;
use ggmath::prelude::*;

use crate::gfx::{
    vertex_layout::VertexLayout,
    vertex_list::{VertexList, VertexListInput},
};

/// A list of vertices and indices that represent a shape in 3D space using triangles.
pub struct ShapeTriangles {
    /// The positions of the vertices.
    positions: Vec<Vector3<f32>>,
    /// The normals of the vertices.
    normals: Vec<Vector3<f32>>,
    /// The colors of the vertices.
    colors: Vec<Vector4<f32>>,
    /// The indices of the vertices that make up the triangles.
    indices: Vec<u32>,
}

impl ShapeTriangles {
    /// Creates a new `ShapeTriangles` with the given vertices and indices.
    /// # Safety
    /// This function is unsafe because it does not check if the indices are valid.
    /// It also does not check other arguments.
    pub(crate) unsafe fn new_unchecked(
        positions: Vec<Vector3<f32>>,
        normals: Vec<Vector3<f32>>,
        colors: Vec<Vector4<f32>>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            positions,
            normals,
            colors,
            indices,
        }
    }

    /// Creates a new `ShapeTriangles` with the given vertices and indices.
    pub fn new(
        positions: Vec<Vector3<f32>>,
        normals: Vec<Vector3<f32>>,
        colors: Vec<Vector4<f32>>,
        indices: Vec<u32>,
    ) -> Result<Self> {
        // Ensure that the positions, normals, and colors have the same length.
        if positions.len() != normals.len() || positions.len() != colors.len() {
            anyhow::bail!("The positions, normals, and colors must have the same length.");
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

        Ok(unsafe { Self::new_unchecked(positions, normals, colors, indices) })
    }

    /// Appends the triangles from another shape to this shape.
    pub fn append(&mut self, other: &mut ShapeTriangles) {
        let offset = self.positions.len() as u32;
        self.positions.append(&mut other.positions);
        self.normals.append(&mut other.normals);
        self.colors.append(&mut other.colors);
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
    pub fn into_vertex_list(self, layout: VertexLayout) -> Result<VertexList> {
        VertexList::new(
            layout,
            &[
                VertexListInput::Position(&self.positions),
                VertexListInput::Normal(&self.normals),
                VertexListInput::Color(&self.colors),
            ],
            Some(self.indices),
        )
    }
}

/// A trait for shapes that can be converted to a list of triangle vertices.
pub trait ShapeToTriangles {
    /// Converts the shape to a list of triangle vertices.
    fn to_triangles(&self) -> ShapeTriangles;
}

// Implement ShapeToTriangles for Vec<T> where T: ShapeToTriangles.
impl<T> ShapeToTriangles for Vec<T>
where
    T: ShapeToTriangles,
{
    fn to_triangles(&self) -> ShapeTriangles {
        let mut triangles =
            ShapeTriangles::new(Vec::new(), Vec::new(), Vec::new(), Vec::new()).unwrap();

        for shape in self {
            let mut shape_triangles = shape.to_triangles();
            triangles.append(&mut shape_triangles);
        }

        triangles
    }
}

/// A rectangle shape in 3D space.
pub struct Rectangle {
    /// The center of the rectangle.
    pub center: Vector3<f32>,
    /// The forward direction of the rectangle.
    pub forward: Vector3<f32>,
    /// The up direction of the rectangle.
    pub up: Vector3<f32>,
    /// The size of the rectangle.
    pub size: Vector2<f32>,
    /// The color of the rectangle.
    pub color: Vector4<f32>,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(
        center: Vector3<f32>,
        forward: Vector3<f32>,
        up: Vector3<f32>,
        size: Vector2<f32>,
        color: Vector4<f32>,
    ) -> Self {
        // Normalize the forward and up vectors.
        let forward = forward.normalized();
        let up = up.normalized();
        Self {
            center,
            forward,
            up,
            size,
            color,
        }
    }

    /// Creates a new rectangle facing the positive Z-axis.
    pub fn new_z(center: Vector3<f32>, size: Vector2<f32>, color: Vector4<f32>) -> Self {
        Self::new(center, Vector3::unit_z(), Vector3::unit_y(), size, color)
    }
}

impl ShapeToTriangles for Rectangle {
    fn to_triangles(&self) -> ShapeTriangles {
        // Calculate the half size.
        let half_size = self.size / 2.0;

        // Calculate the forward, up, and right vectors.
        let forward = self.forward;
        let up = self.up;
        let right = forward.cross(&up);

        // Calculate the positions, normals, colors, and indices.
        let positions = vec![
            self.center + right * half_size.x() + up * half_size.y(),
            self.center + right * half_size.x() - up * half_size.y(),
            self.center - right * half_size.x() - up * half_size.y(),
            self.center - right * half_size.x() + up * half_size.y(),
        ];
        let normals = vec![forward, forward, forward, forward];
        let colors = vec![self.color, self.color, self.color, self.color];
        let indices = vec![0, 1, 2, 0, 2, 3];

        unsafe { ShapeTriangles::new_unchecked(positions, normals, colors, indices) }
    }
}

/// A box shape in 3D space.
pub struct Box {
    /// The center of the box.
    pub center: Vector3<f32>,
    /// The forward direction of the box.
    pub forward: Vector3<f32>,
    /// The up direction of the box.
    pub up: Vector3<f32>,
    /// The size of the box.
    pub size: Vector3<f32>,
    /// The color of the box.
    pub color: Vector4<f32>,
}

impl Box {
    /// Creates a new box.
    pub fn new(
        center: Vector3<f32>,
        forward: Vector3<f32>,
        up: Vector3<f32>,
        size: Vector3<f32>,
        color: Vector4<f32>,
    ) -> Self {
        // Normalize the forward and up vectors.
        let forward = forward.normalized();
        let up = up.normalized();
        Self {
            center,
            forward,
            up,
            size,
            color,
        }
    }

    /// Creates a new box facing the positive Z-axis.
    pub fn new_z(center: Vector3<f32>, size: Vector3<f32>, color: Vector4<f32>) -> Self {
        Self::new(center, Vector3::unit_z(), Vector3::unit_y(), size, color)
    }
}

impl ShapeToTriangles for Box {
    fn to_triangles(&self) -> ShapeTriangles {
        // Calculate the half size.
        let half_size = self.size / 2.0;

        // Calculate the forward, up, and right vectors.
        let forward = self.forward;
        let up = self.up;
        let right = forward.cross(&up);

        // Create rectangle shapes for each side of the box.
        let rectangles = vec![
            Rectangle::new(
                self.center + forward * half_size.z(),
                forward,
                up,
                self.size.xy(),
                self.color,
            ),
            Rectangle::new(
                self.center - forward * half_size.z(),
                -forward,
                up,
                self.size.xy(),
                self.color,
            ),
            Rectangle::new(
                self.center + up * half_size.y(),
                up,
                forward,
                self.size.xz(),
                self.color,
            ),
            Rectangle::new(
                self.center - up * half_size.y(),
                -up,
                forward,
                self.size.xz(),
                self.color,
            ),
            Rectangle::new(
                self.center + right * half_size.x(),
                right,
                up,
                self.size.yz(),
                self.color,
            ),
            Rectangle::new(
                self.center - right * half_size.x(),
                -right,
                up,
                self.size.yz(),
                self.color,
            ),
        ];

        // Convert the rectangles to triangles.
        rectangles.to_triangles()
    }
}
