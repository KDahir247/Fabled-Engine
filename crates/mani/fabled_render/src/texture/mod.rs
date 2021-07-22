pub use codecs::*;
use common::*;
pub use container::*;
pub use core::*;
pub use texture_data::*;
pub use texture_sampler::*;
pub use texture_view_dimension::*;
pub use EXT::*;

mod storage;
mod synthesizer;

//Clean up
pub mod EXT;
pub mod codecs;
mod common;
mod container;
mod texture_aspect;
mod texture_data;
mod texture_dimension;
mod texture_sample_type;
mod texture_sampler;
mod texture_view_dimension;

#[cfg(test)]
mod data_alignment_test {
    use crate::texture::container::*;
    use crate::texture::*;

    #[test]
    fn data_alignment() {
        let border_color = std::mem::size_of::<BorderColor>();
        assert_eq!(border_color & (border_color - 1), 0);

        let extent3d = std::mem::size_of::<Extent3d>();
        assert_eq!(extent3d & (extent3d - 1), 0);

        let texture_data = std::mem::size_of::<Texture>();
        assert_eq!(texture_data & (texture_data - 1), 0);

        let texture_dimension = std::mem::size_of::<TextureViewDimension>();
        assert_eq!(texture_dimension & (texture_dimension - 1), 0);

        let sampler = std::mem::size_of::<TextureSampler>();
        assert_eq!(sampler & (sampler - 1), 0);

        let color_space = std::mem::size_of::<ColorSpace>();
        assert_eq!(color_space & (color_space - 1), 0);

        let flip_axis = std::mem::size_of::<flip_axis::FlipAxis>();
        assert_eq!(flip_axis & (flip_axis - 1), 0);

        let texture_access = std::mem::size_of::<texture_access::StorageTextureAccess>();
        assert_eq!(texture_access & (texture_access - 1), 0);

        let texture_aspect = std::mem::size_of::<texture_aspect::TextureAspect>();
        assert_eq!(texture_aspect & (texture_aspect - 1), 0);

        let sample_type = std::mem::size_of::<texture_sample_type::TextureSampleType>();
        assert_eq!(sample_type & (sample_type - 1), 0);

        let texture_dimension = std::mem::size_of::<texture_dimension::TextureDimension>();
        assert_eq!(texture_dimension & (texture_dimension - 1), 0);
    }
}
