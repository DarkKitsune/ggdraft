use std::collections::HashMap;

use anyhow::Result;
use ggmath::prelude::*;
use image::GenericImageView;

/// A GL texture.
pub struct Texture {
    handle: u32,
    texture_type: TextureType,
    dimensions: Vec<Vector2<u32>>,
    regions: Option<HashMap<String, TextureRegion>>,
    glyphs: Option<HashMap<char, TextureGlyph>>,
}

impl !Send for Texture {}
impl !Sync for Texture {}

impl Texture {
    /// Create a new texture from an image.
    /// # Safety
    /// This function is unsafe because it should only be used on the main thread.
    pub(crate) unsafe fn __from_image(
        name: impl AsRef<str>,
        texture_type: TextureType,
        lods: &[image::DynamicImage],
        regions: Option<HashMap<String, TextureRegion>>,
        glyphs: Option<HashMap<char, TextureGlyph>>,
    ) -> Result<Self> {
        let name = name.as_ref();

        // Ensure that there is at least one LOD.
        if lods.is_empty() {
            anyhow::bail!("No LODs provided for texture {}", name);
        }

        // Ensure that each LOD has the same format as the first LOD.
        for (i, lod) in lods.iter().enumerate() {
            if lod.color() != lods[0].color() {
                anyhow::bail!(
                    "Failed to load {}: LOD {} has color type {:?} different from LOD 0 {:?}",
                    name,
                    i,
                    lod.color(),
                    lods[0].color()
                );
            }
        }

        // Ensure that each LOD has the same or smaller dimensions than the previous LOD.
        for i in 1..lods.len() {
            let (prev_width, prev_height) = lods[i - 1].dimensions();
            let (width, height) = lods[i].dimensions();
            if width > prev_width || height > prev_height {
                anyhow::bail!(
                    "Failed to load {}: LOD {} has dimensions ({}, {}) larger than LOD {} ({}, {})",
                    name,
                    i,
                    width,
                    height,
                    i - 1,
                    prev_width,
                    prev_height
                );
            }
        }

        // Get the dimensions of each LOD.
        let dimensions = lods
            .iter()
            .map(|lod| vector!(lod.width() as u32, lod.height() as u32))
            .collect();

        // Create the texture.
        unsafe {
            let mut handle = 0;
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut handle);
            gl::TextureStorage2D(
                handle,
                lods.len() as i32,
                gl::RGBA8,
                lods[0].width() as i32,
                lods[0].height() as i32,
            );

            for (i, lod) in lods.iter().enumerate() {
                let lod = lod.to_rgba8();
                gl::TextureSubImage2D(
                    handle,
                    i as i32,
                    0,
                    0,
                    lod.width() as i32,
                    lod.height() as i32,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    lod.as_ptr() as *const _,
                );
            }

            Ok(Self {
                handle,
                texture_type,
                dimensions,
                regions,
                glyphs,
            })
        }
    }

    /// Get the dimensions of the texture at the given LOD.
    /// Returns `None` if the LOD does not exist.
    pub fn dimensions(&self, lod: usize) -> Option<Vector2<u32>> {
        self.dimensions.get(lod).copied()
    }

    /// Get the number of LODs in this texture.
    pub fn lod_count(&self) -> usize {
        self.dimensions.len()
    }

    /// Get the given texture region.
    /// Returns `None` if the region does not exist.
    pub fn region(&self, name: impl AsRef<str>) -> Option<&TextureRegion> {
        self.regions.as_ref()?.get(name.as_ref())
    }

    /// Get the min and max texture coordinates of the given texture region.
    /// Returns `None` if the region does not exist.
    pub fn region_tex_coord(&self, name: impl AsRef<str>) -> Option<(Vector2<f32>, Vector2<f32>)> {
        let region = self.region(name)?;
        let dimensions = self.dimensions(0)?.convert_to::<f32>().unwrap();

        let min = region.min_pixel().convert_to::<f32>().unwrap() / dimensions;
        let max = region.max_pixel().convert_to::<f32>().unwrap() / dimensions;

        // Flip the Y axis.
        Some((
            vector!(min.x(), max.y()),
            vector!(max.x(), min.y()),
        ))
    }

    /// Get the min and max LOD levels of the given texture region.
    /// Returns `None` if the region does not exist.
    pub fn region_lod_levels(&self, name: impl AsRef<str>) -> Option<(u32, u32)> {
        let region = self.region(name)?;
        Some((region.min_lod(), region.max_lod()))
    }

    /// Get the given character glyph in this texture.
    /// Returns `None` if the glyph does not exist.
    pub fn glyph(&self, character: char) -> Option<&TextureGlyph> {
        self.glyphs.as_ref()?.get(&character)
    }

    /// Get the min and max texture coordinates of the given character glyph.
    /// Returns `None` if the glyph does not exist.
    pub fn glyph_tex_coord(&self, character: char) -> Option<(Vector2<f32>, Vector2<f32>)> {
        let glyph = self.glyph(character)?;
        let dimensions = self.dimensions(0)?.convert_to::<f32>().unwrap();

        let min = glyph.region().min_pixel().convert_to::<f32>().unwrap() / dimensions;
        let max = glyph.region().max_pixel().convert_to::<f32>().unwrap() / dimensions;

        // Flip the Y axis.
        Some((
            vector!(min.x(), max.y()),
            vector!(max.x(), min.y()),
        ))
    }

    /// Get the min and max LOD levels of the given character glyph.
    /// Returns `None` if the glyph does not exist.
    pub fn glyph_lod_levels(&self, character: char) -> Option<(u32, u32)> {
        let glyph = self.glyph(character)?;
        Some((glyph.region().min_lod(), glyph.region().max_lod()))
    }

    /// Get the GL handle.
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Get this texture's type.
    pub fn texture_type(&self) -> TextureType {
        self.texture_type
    }

    /// Get a `TextureView` to the entirety of this texture.
    pub fn full_view(&self) -> TextureView {
        TextureView {
            texture_handle: self.handle,
            texture_type: self.texture_type,
            min: Vector::zero(),
            max: Vector::one(),
        }
    }

    /// Get a `TextureView` to the entirety of this texture at the given LOD.
    pub fn lod_view(&self, lod: usize) -> Option<TextureView> {
        let dimensions = self.dimensions(lod)?;
        let to_tex_coord_and_levels = vector!(dimensions.x() as f32, dimensions.y() as f32, 1.0);

        Some(TextureView {
            texture_handle: self.handle,
            texture_type: self.texture_type,
            min: vector!(0.0, 0.0, lod as f32) / to_tex_coord_and_levels,
            max: vector!(1.0, 1.0, lod as f32) / to_tex_coord_and_levels,
        })
    }

    /// Get a `TextureView` to a region of this texture.
    /// Returns `None` if the region does not exist.
    pub fn region_view(&self, name: impl AsRef<str>) -> Option<TextureView> {
        let name = name.as_ref();
        let (min, max) = self.region_tex_coord(name)?;
        let (min_lod, max_lod) = self.region_lod_levels(name)?;

        Some(TextureView {
            texture_handle: self.handle,
            texture_type: self.texture_type,
            min: min.append(min_lod as f32),
            max: max.append(max_lod as f32),
        })
    }

    /// Get a `TextureView` to a character glyph in this texture.
    /// Returns `None` if the glyph does not exist.
    pub fn glyph_view(&self, character: char) -> Option<TextureView> {
        let (min, max) = self.glyph_tex_coord(character)?;
        let (min_lod, max_lod) = self.glyph_lod_levels(character)?;

        Some(TextureView {
            texture_handle: self.handle,
            texture_type: self.texture_type,
            min: min.append(min_lod as f32),
            max: max.append(max_lod as f32),
        })
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            // Delete the texture only if self.handle is not 0.
            if self.handle != 0 {
                gl::DeleteTextures(1, &self.handle);
            }
        }
    }
}

/// Represents a type of texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureType {
    Invalid,
    Color,
    Normal,
    Metallic,
    Roughness,
    Emissive,
}

impl TextureType {
    /// Convert to a texture unit index.
    pub(crate) fn texture_unit_index(&self) -> u32 {
        match self {
            TextureType::Invalid => panic!("Invalid texture type"),
            TextureType::Color => 0,
            TextureType::Normal => 1,
            TextureType::Metallic => 2,
            TextureType::Roughness => 3,
            TextureType::Emissive => 4,
        }
    }
}

/// Represents a region within a texture.
/// The X and Y axes correspond to the image's pixels.
/// The Z axis corresponds to the LOD level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureRegion {
    min: Vector3<i32>,
    max: Vector3<i32>,
}

impl TextureRegion {
    /// Create a new texture region with the given top-left coordinate, size, and LOD levels.
    pub const fn new(
        top_left: Vector2<i32>,
        size: Vector2<i32>,
        min_level: u32,
        levels: u32,
    ) -> Self {
        let min = top_left.append(min_level as i32);
        let max = vector!(
            top_left.x() + size.x(),
            top_left.y() + size.y(),
            min.z() + levels.saturating_sub(1) as i32
        );
        Self::from_min_max(min, max)
    }

    /// Create a new texture region with the given minimum and maximum coordinates & LOD levels.
    pub const fn from_min_max(min: Vector3<i32>, max: Vector3<i32>) -> Self {
        const fn const_max(a: i32, b: i32) -> i32 {
            if a > b {
                a
            } else {
                b
            }
        }

        // Ensure that the LOD levels are non-negative.
        let min = vector!(min.x(), min.y(), const_max(min.z(), 0));
        let max = vector!(max.x(), max.y(), const_max(max.z(), 0));

        Self { min, max }
    }

    /// Get the minimum coordinates.
    pub const fn min(&self) -> Vector3<i32> {
        self.min
    }

    /// Get the maximum coordinates.
    pub const fn max(&self) -> Vector3<i32> {
        self.max
    }

    /// Get the minimum pixel coordinates.
    pub const fn min_pixel(&self) -> Vector2<i32> {
        self.min.xy()
    }

    /// Get the maximum pixel coordinates.
    pub const fn max_pixel(&self) -> Vector2<i32> {
        self.max.xy()
    }

    /// Get the pixel size (width, height) of this region.
    pub const fn pixel_size(&self) -> Vector2<i32> {
        vector!(self.max.x() - self.min.x(), self.max.y() - self.min.y())
    }

    /// Get the aspect ratio of this region.
    /// This is the width divided by the height.
    pub const fn aspect_ratio(&self) -> f32 {
        let size = vector!(
            (self.max.x() - self.min.x()) as f32,
            (self.max.y() - self.min.y()) as f32
        );
        size.x() / size.y()
    }

    /// Get the minimum LOD level.
    pub const fn min_lod(&self) -> u32 {
        self.min.z() as u32
    }

    /// Get the maximum LOD level.
    pub const fn max_lod(&self) -> u32 {
        self.max.z() as u32
    }

    /// Get the number of LOD levels in this region.
    pub const fn lod_count(&self) -> u32 {
        (self.max.z() - self.min.z()) as u32
    }
}

/// Represents a glyph within a texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureGlyph {
    /// The texture region containing the glyph.
    region: TextureRegion,
    /// The number of pixels to advance after rendering the glyph at a scale of 1.
    advance: i32,
}

impl TextureGlyph {
    /// Create a new texture glyph with the given region and advance pixels.
    pub const fn new(region: TextureRegion, advance: i32) -> Self {
        Self { region, advance }
    }

    /// Get the texture region containing the glyph.
    pub const fn region(&self) -> TextureRegion {
        self.region
    }

    /// Get the number of pixels to advance after rendering the glyph at a scale of 1.
    pub const fn advance(&self) -> i32 {
        self.advance
    }
}

impl AsRef<TextureRegion> for TextureGlyph {
    fn as_ref(&self) -> &TextureRegion {
        &self.region
    }
}

/// Represents a view of a specific region in a texture, for sampling.
/// The X and Y axes are the texture coordinates.
/// The Z axis is the range of LOD levels to sample (0.0 to 1.0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureView {
    texture_handle: u32,
    texture_type: TextureType,
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl !Send for TextureView {}
impl !Sync for TextureView {}

impl TextureView {
    /// Get the texture handle.
    pub fn handle(&self) -> u32 {
        self.texture_handle
    }

    /// Get the texture type.
    pub fn texture_type(&self) -> TextureType {
        self.texture_type
    }

    /// Get the minimum coordinates.
    pub fn min(&self) -> Vector3<f32> {
        self.min
    }

    /// Get the maximum coordinates.
    pub fn max(&self) -> Vector3<f32> {
        self.max
    }

    /// Get the minimum texture coordinates.
    pub fn min_tex_coord(&self) -> Vector2<f32> {
        self.min.xy()
    }

    /// Get the maximum texture coordinates.
    pub fn max_tex_coord(&self) -> Vector2<f32> {
        self.max.xy()
    }

    /// Get the minimum LOD level (0.0 to 1.0)
    pub fn min_lod(&self) -> f32 {
        self.min.z()
    }

    /// Get the maximum LOD level (0.0 to 1.0)
    pub fn max_lod(&self) -> f32 {
        self.max.z()
    }
}

impl Default for TextureView {
    fn default() -> Self {
        Self {
            texture_handle: 0,
            texture_type: TextureType::Invalid,
            min: Vector::zero(),
            max: Vector::one(),
        }
    }
}
