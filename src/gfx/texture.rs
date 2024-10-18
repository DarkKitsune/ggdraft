use anyhow::Result;
use ggmath::prelude::*;
use image::GenericImageView;

/// A GL texture.
pub struct Texture {
    handle: u32,
    texture_type: TextureType,
    dimensions: Vec<Vector2<u32>>,
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
            })
        }
    }

    /// Get the dimensions of the texture at the given LOD.
    /// Returns `None` if the LOD does not exist.
    pub fn dimensions(&self, lod: usize) -> Option<Vector2<u32>> {
        self.dimensions.get(lod).copied()
    }

    /// Get the GL handle.
    pub fn handle(&self) -> u32 {
        self.handle
    }

    /// Get this texture's type.
    pub fn texture_type(&self) -> TextureType {
        self.texture_type
    }

    /// Get a `TextureView` pointing to this texture.
    pub fn view(&self) -> TextureView {
        TextureView {
            texture_handle: self.handle,
            texture_type: self.texture_type,
        }
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

/// Represents a view of a texture for use in rendering.
pub struct TextureView {
    texture_handle: u32,
    texture_type: TextureType,
}

impl TextureView {
    /// Get the texture handle.
    pub fn handle(&self) -> u32 {
        self.texture_handle
    }

    /// Get the texture type.
    pub fn texture_type(&self) -> TextureType {
        self.texture_type
    }
}

impl Default for TextureView {
    fn default() -> Self {
        Self {
            texture_handle: 0,
            texture_type: TextureType::Invalid,
        }
    }
}
