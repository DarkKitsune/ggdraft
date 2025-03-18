use std::{char, collections::HashMap, fmt::Display};

use ggmath::prelude::*;

use crate::{
    app::app_prelude::*,
    gfx::{
        gfx_cache::{CacheHandle, GfxCache},
        texture::TextureGlyph,
        vertex_layout::VertexLayout,
    },
    svector,
};

use super::{
    alignment::AxisAlignment,
    orientation::{HasOrientation, Orientation},
    shape::{Rectangle, ShapeToTriangles, ShapeTriangles},
};

pub const FALLBACK_GLYPH: char = '?';

/// A text object that can be rendered in 2D or 3D space.
pub struct Text {
    /// The orientation of the text.
    orientation: Orientation,
    /// The alignment mode of the text.
    alignment: TextAlignment,
    /// The font texture from which glyphs are rendered.
    font_texture: CacheHandle,
    /// The color of the text.
    color: Vector4<f32>,
    /// The text to render.
    text: String,
}

impl Text {
    /// Create a new text object with the given orientation
    pub fn new_with_orientation(
        orientation: Orientation,
        alignment: TextAlignment,
        font_texture: CacheHandle,
        color: Vector4<f32>,
        text: impl Display,
    ) -> Self {
        Self {
            orientation,
            alignment,
            font_texture,
            color,
            text: text.to_string(),
        }
    }

    /// Create a new text object.
    pub fn new(
        position: Vector3<f32>,
        rotation: Quaternion<f32>,
        scale: Vector2<f32>,
        alignment: TextAlignment,
        font_texture: CacheHandle,
        color: Vector4<f32>,
        text: impl Display,
    ) -> Self {
        Self::new_with_orientation(
            Orientation::new(position, rotation, scale.append(1.0)),
            alignment,
            font_texture,
            color,
            text,
        )
    }

    /// Create a new text object with the default orientation at 0,0,0.
    pub fn new_simple(
        alignment: TextAlignment,
        font_texture: CacheHandle,
        color: Vector4<f32>,
        text: impl Display,
    ) -> Self {
        Self::new_with_orientation(Orientation::default(), alignment, font_texture, color, text)
    }

    /// Get the orientation of the text.
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    /// Get the alignment of the text.
    pub fn alignment(&self) -> TextAlignment {
        self.alignment
    }

    /// Get the font texture of the text.
    pub fn font_texture(&self) -> CacheHandle {
        self.font_texture.clone()
    }

    /// Get the color of the text.
    pub fn color(&self) -> Vector4<f32> {
        self.color
    }

    /// Get the text string to be rendered.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the orientation of the text.
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    /// Set the alignment of the text.
    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }

    /// Set the font texture of the text.
    pub fn set_font_texture(&mut self, font_texture: CacheHandle) {
        self.font_texture = font_texture;
    }

    /// Set the color of the text.
    pub fn set_color(&mut self, color: Vector4<f32>) {
        self.color = color;
    }

    /// Set the text string to be rendered.
    pub fn set_text(&mut self, text: impl Display) {
        self.text = text.to_string();
    }

    /// Build a vertex layout suitable for text rendering
    pub fn build_vertex_layout(layout: VertexLayout) -> VertexLayout {
        layout.with_position().with_color().with_tex_coord()
    }

    /// The vertex shader for rendering text
    pub fn vertex_shader(
        inputs: &ShaderInputs,
        parameters: &mut ShaderParameters,
        outputs: &mut ShaderOutputs,
    ) -> Result<()> {
        // Get the vertex inputs
        let position = inputs
            .get(VertexInput::Position)
            .expect("Vertex layout does not provide position");
        let tex_coord = inputs
            .get(VertexInput::TexCoord)
            .expect("Vertex layout does not provide texture coordinates");
        let color = inputs.get(VertexInput::Color).unwrap_or(svector!(1; 4));

        // Get the matrices
        let model_matrix = parameters.get_model_matrix();
        let view_matrix = parameters.get_view_matrix();
        let projection_matrix = parameters.get_projection_matrix();

        // Transform the vertex position into clip space
        let position = model_matrix * position.append(1.0);
        let position = view_matrix * position;
        let position = projection_matrix * position;

        // Set the output vertex position
        outputs.set_vertex_position(position);

        // Forward the texture coordinates and color to the fragment shader
        outputs.set("tex_coord", tex_coord)?;
        outputs.set("color", color)?;

        Ok(())
    }

    /// The fragment shader for rendering text
    pub fn fragment_shader(
        inputs: &ShaderInputs,
        parameters: &mut ShaderParameters,
        outputs: &mut ShaderOutputs,
    ) -> Result<()> {
        // Get the fragment inputs
        let tex_coord = inputs
            .get("tex_coord")
            .expect("Vertex shader did not provide texture coordinates");
        let color = inputs
            .get("color")
            .expect("Vertex shader did not provide color");

        // Get the font texture
        let font_texture = parameters.get::<TextureView>("font_texture");

        // Sample the font texture
        // TODO: Implement LODs
        let sampled_color = font_texture.sample(tex_coord, 0.0);

        // Multiply the sampled color with the input color
        let final_color = sampled_color * color;

        // Output the final color
        outputs.set_fragment_color(final_color);

        Ok(())
    }

    /// Build a hashmap of glyphs from a slice of strings, and grid dimensions
    pub fn build_glyph_map<'a>(
        // The rows of the grid
        row_contents: impl IntoIterator<Item = &'a str>,
        // Pixel coordinates of the top-left corner of the grid
        grid_source_position: Vector2<i32>,
        // Pixel size of each glyph in the source image (before cropping)
        glyph_source_size: Vector2<i32>,
        // How much to crop from the edges (assumes the glyph is centered)
        // The X component is applied to the left and right sides
        // The Y component is applied to the top and bottom sides
        glyph_crop: Vector2<i32>,
    ) -> HashMap<char, TextureGlyph> {
        let mut glyphs = HashMap::new();
        let cropped_glyph_size = glyph_source_size - glyph_crop * 2;

        // Iterate over the rows
        for (y, row) in row_contents.into_iter().enumerate() {
            // Iterate over the characters in the row
            for (x, character) in row.chars().enumerate() {
                // Calculate the glyph's source region
                let min = grid_source_position
                    + vector!(x as i32, y as i32) * glyph_source_size
                    + glyph_crop;
                let region = TextureRegion::new(min, cropped_glyph_size, 0, 1);

                // Create the glyph and add it to the map
                let glyph = TextureGlyph::new(region, cropped_glyph_size.x());
                glyphs.insert(character, glyph);
            }
        }

        glyphs
    }
}

impl HasOrientation for Text {
    fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    fn orientation_mut(&mut self) -> &mut Orientation {
        &mut self.orientation
    }
}

impl ShapeToTriangles for Text {
    fn to_triangles(&self, cache: &GfxCache) -> ShapeTriangles {
        // TODO: Implement aspect ratio calculations vv
        let glyph_aspect_ratio = 1.0;
        let texture = cache
            .get_texture(&self.font_texture)
            .expect("Texture not found");

        // First we map the characters to a 2D grid of `TextureView`s
        // Also calculate the size of the grid for alignment calculations
        let mut view_grid = Vec::new();
        let mut row_vec = Vec::new();
        let mut row = 0;
        let mut column = 0;
        let mut widest_row_width = 0;
        for character in self.text.chars() {
            // Handle newlines
            if character == '\n' {
                // Update the widest row width if the current row is the widest
                widest_row_width = Ord::max(widest_row_width, column);

                // Push the row and start a new one
                view_grid.push(row_vec.clone());
                row_vec.clear();
                row += 1;
                column = 0;

                continue;
            }

            // Get the texture view for the glyph and add it to the row
            let view = texture.glyph_view(character).unwrap_or_else(|| {
                texture
                    .glyph_view(FALLBACK_GLYPH)
                    .expect("Fallback glyph not found")
            });
            row_vec.push(view);

            // Update the column
            column += 1;
        }

        // Update the widest row width if the last row was the widest
        widest_row_width = Ord::max(widest_row_width, column);

        // Push the last row if it's not empty
        if !row_vec.is_empty() {
            view_grid.push(row_vec);
            // We still need to increment the row counter to calculate the grid size correctly
            row += 1;
        }

        // Calculate the glyph size, and grid size (in glyphs)
        let scale = self.scale().xy();
        let glyph_size = vector!(
            f32::min(glyph_aspect_ratio, 1.0),
            f32::min(1.0 / glyph_aspect_ratio, 1.0),
        ) * scale;
        let grid_size = vector!(widest_row_width as f32, row as f32);

        // Create a grid of rectangles for generating the text mesh.
        // The top-left corner of the grid is offset from Self::orientation.position by (grid_size as f32) / 2
        let rotation_matrix = self.get_rotation_matrix();
        let x_step = rotation_matrix.x_axis() * glyph_size.x();
        let y_step = rotation_matrix.y_axis() * glyph_size.y();
        let alignment_offset = vector!(
            match self.alignment.horizontal {
                AxisAlignment::Min => 0.0,
                AxisAlignment::Center => -0.5,
                AxisAlignment::Max => -1.0,
            },
            match self.alignment.vertical {
                AxisAlignment::Min => 0.0,
                AxisAlignment::Center => -0.5,
                AxisAlignment::Max => -1.0,
            },
        ) * grid_size;
        let base_position = self.position()
            + x_step * alignment_offset.x()
            + y_step * alignment_offset.y();
        let mut rectangles = Vec::new();
        
        // Iterate over the grid and create a rectangle for each view
        for (y, row) in view_grid.iter().enumerate() {
            for (x, view) in row.iter().enumerate() {
                // Calculate the center of the corresponding rectangle
                let position =
                    base_position + x_step * (x as f32 + 0.5) + y_step * (y as f32 + 0.5);

                // Create the rectangle
                let rectangle = Rectangle::new(position, glyph_size, self.rotation(), self.color())
                    .with_texture_view_coords(view);
                rectangles.push(rectangle);
            }
        }

        // Convert the rectangles to triangles
        rectangles.to_triangles(cache)
    }
}

/// Represents a text alignment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextAlignment {
    /// The alignment mode on the horizontal axis.
    pub horizontal: AxisAlignment,
    /// The alignment mode on the vertical axis.
    pub vertical: AxisAlignment,
}

impl TextAlignment {
    pub const TOP_LEFT: Self = Self {
        horizontal: AxisAlignment::Min,
        vertical: AxisAlignment::Max,
    };

    pub const TOP_CENTER: Self = Self {
        horizontal: AxisAlignment::Center,
        vertical: AxisAlignment::Max,
    };

    pub const TOP_RIGHT: Self = Self {
        horizontal: AxisAlignment::Max,
        vertical: AxisAlignment::Max,
    };

    pub const CENTER_LEFT: Self = Self {
        horizontal: AxisAlignment::Min,
        vertical: AxisAlignment::Center,
    };

    pub const CENTER: Self = Self {
        horizontal: AxisAlignment::Center,
        vertical: AxisAlignment::Center,
    };

    pub const CENTER_RIGHT: Self = Self {
        horizontal: AxisAlignment::Max,
        vertical: AxisAlignment::Center,
    };

    pub const BOTTOM_LEFT: Self = Self {
        horizontal: AxisAlignment::Min,
        vertical: AxisAlignment::Min,
    };

    pub const BOTTOM_CENTER: Self = Self {
        horizontal: AxisAlignment::Center,
        vertical: AxisAlignment::Min,
    };

    pub const BOTTOM_RIGHT: Self = Self {
        horizontal: AxisAlignment::Max,
        vertical: AxisAlignment::Min,
    };
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::TOP_LEFT
    }
}
