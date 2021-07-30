use crate::texture::container::{ColorType, Extent3d};

#[derive(Debug)]
#[repr(align(64))]
pub struct Texture {
    pub data: Vec<u8>,
    pub size: Extent3d,
    pub sample_count: u32,
    pub mip_level: u32,
    pub color_type: ColorType,
    /// Bytes per row of the image.
    pub rows_per_image: u32,
}
