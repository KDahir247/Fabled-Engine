use crate::texture::texture_dimension::TextureDimension;
use crate::FlipAxis;

pub mod dds;
pub mod jpg;
pub mod png;
pub mod tiff;

pub use dds::*;
pub use jpg::*;
pub use png::*;
pub use tiff::*;

#[derive(Clone, Debug)]
pub struct TextureDescriptor {
    pub flip_axis: FlipAxis,
    pub dimensions: TextureDimension,
    pub format: u32,
    pub usage: u32,
    //No sure about mip_level since i haven't played around with it
    //same goes with sample count
}

impl Default for TextureDescriptor {
    // Explicit default for format and usage so it doesn't = to 0
    fn default() -> Self {
        Self {
            flip_axis: Default::default(),
            dimensions: Default::default(),
            format: 18,
            usage: 6,
        }
    }
}

#[cfg(test)]
pub mod codecs_test {
    use crate::TextureDescriptor;

    #[test]
    fn data_alignment() {
        let tex_desc = std::mem::size_of::<TextureDescriptor>();
        assert_eq!(tex_desc & (tex_desc - 1), 0);
    }
}
