mod dds;
mod hdr;
mod jpg;
mod png;
mod tiff;

pub use dds::*;
pub use hdr::*;
pub use jpg::*;
pub use png::*;
pub use tiff::*;

use crate::texture::container::FlipAxis;

#[derive(Clone, Debug)]
pub struct TextureDescriptor {
    pub flip_axis: FlipAxis,
}

impl Default for TextureDescriptor {
    // Explicit default for format and usage so it doesn't = to 0
    fn default() -> Self {
        Self {
            flip_axis: Default::default(),
        }
    }
}

#[cfg(test)]
mod codecs_test {
    use crate::texture::codecs::TextureDescriptor;
    #[test]
    fn data_size() {
        let tex_desc_size = std::mem::size_of::<TextureDescriptor>();
        assert_eq!(tex_desc_size & (tex_desc_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let tex_desc_alignment = std::mem::align_of::<TextureDescriptor>();
        assert_eq!(tex_desc_alignment & (tex_desc_alignment - 1), 0);
    }
}
