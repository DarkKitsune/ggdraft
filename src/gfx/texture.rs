use anyhow::Result;
use ggmath::prelude::*;
use image::GenericImageView;

/// A GL texture.
pub struct Texture {
    handle: u32,
    dimensions: Vec<Vector2<u32>>,
}

impl !Send for Texture {}
impl !Sync for Texture {}

impl Texture {
    /// Create a new texture from an image.
    /// # Safety
    /// This function is unsafe because it should only be used on the main thread.
    pub(crate) unsafe fn __from_image(name: impl AsRef<str>, lods: &[image::DynamicImage]) -> Result<Self> {
        let name = name.as_ref();

        // Ensure that there is at least one LOD.
        if lods.is_empty() {
            anyhow::bail!("No LODs provided for texture {}", name);
        }

        // Get the format of the first LOD.
        let format = match lods[0].color() {
            image::ColorType::Rgba8 => gl::RGBA8,
            image::ColorType::Rgb8 => gl::RGB8,
            _ => anyhow::bail!("Unsupported color type for texture {}", name),
        };

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
            .map(|lod| {
                let (width, height) = lod.dimensions();
                vector!(width, height)
            })
            .collect();

        // Create the texture.
        unsafe {
            let mut handle = 0;
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut handle);
            gl::TextureStorage2D(handle, lods.len() as i32, format, lods[0].width() as i32, lods[0].height() as i32);

            for (i, lod) in lods.iter().enumerate() {
                let bytes = lod.as_bytes();
                gl::TextureSubImage2D(
                    handle,
                    i as i32,
                    0,
                    0,
                    lod.width() as i32,
                    lod.height() as i32,
                    format,
                    gl::UNSIGNED_BYTE,
                    bytes.as_ptr() as *const _,
                );
            }

            Ok(Self {
                handle,
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
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}