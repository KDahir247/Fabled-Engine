use crate::texture::container::Extent3d;
use crate::texture::texture_dimension::TextureDimension;
use crate::texture::*;

#[derive(Debug)]
pub struct Texture {
    pub data: Vec<u8>,
    pub size: Extent3d,
    pub format: u32,
    /// # Instruction
    /// * COPY_SRC = 1,
    /// * COPY_DST = 2,
    /// * SAMPLED = 4,
    /// * STORAGE = 8,
    /// * RENDER_ATTACHMENT = 16
    ///
    /// <br/> For more information read [`wgpu::TextureUsage`] documentation
    /// <br/> Can be used as flag to combined instruction
    pub usage: u32,
    pub sample_count: u32,
    pub mip_level: u32,
    pub dimensions: TextureDimension,
    /// Bytes per row of the image.
    pub rows_per_image: u32,
}
