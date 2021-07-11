use crate::texture::container::Extent3d;
use crate::texture::*;

#[derive(Debug)]
pub struct Texture {
    pub data: Vec<u8>,
    pub size: Extent3d,
    pub format: i32,
    pub usage: i32,
    pub sample_count: i32,
    pub mip_level: i32,
    pub dimensions: TextureViewDimension,
}
