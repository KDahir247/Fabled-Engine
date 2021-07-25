pub use border_color::*;
pub use color_space::*;
pub use color_type::*;
pub use extent3d::*;
pub use flip_axis::*;
pub use texture_access::*;
pub use texture_aspect::*;
pub use texture_data::*;
pub use texture_dimension::*;
pub use texture_sample_type::*;
pub use texture_sampler::*;
pub use texture_view_dimension::*;

pub mod border_color;
pub mod color_space;
pub mod color_type;
pub mod extent3d;
pub mod flip_axis;
pub mod texture_access;
pub mod texture_aspect;
pub mod texture_data;
pub mod texture_dimension;
pub mod texture_sample_type;
pub mod texture_sampler;
pub mod texture_view_dimension;

#[cfg(test)]
pub mod data_alignment_test {
    use crate::{
        BorderColor, ColorSpace, ColorType, Extent3d, FlipAxis, StorageTextureAccess, Texture,
        TextureAspect, TextureDimension, TextureSampleType, TextureSampler, TextureViewDimension,
    };

    #[test]
    fn data_alignment() {
        let border_color = std::mem::size_of::<BorderColor>();
        assert_eq!(border_color & (border_color - 1), 0);

        let color_space = std::mem::size_of::<ColorSpace>();
        assert_eq!(color_space & (color_space - 1), 0);

        let extent3d = std::mem::size_of::<Extent3d>();
        assert_eq!(extent3d & (extent3d - 1), 0);

        let flip_axis = std::mem::size_of::<FlipAxis>();
        assert_eq!(flip_axis & (flip_axis - 1), 0);

        let texture_access = std::mem::size_of::<StorageTextureAccess>();
        assert_eq!(texture_access & (texture_access - 1), 0);

        let texture_aspect = std::mem::size_of::<TextureAspect>();
        assert_eq!(texture_aspect & (texture_aspect - 1), 0);

        let texture_data = std::mem::size_of::<Texture>();
        assert_eq!(texture_data & (texture_data - 1), 0);

        let texture_dimension = std::mem::size_of::<TextureDimension>();
        assert_eq!(texture_dimension & (texture_dimension - 1), 0);

        let texture_sample_type = std::mem::size_of::<TextureSampleType>();
        assert_eq!(texture_sample_type & (texture_sample_type - 1), 0);

        let texture_sampler = std::mem::size_of::<TextureSampler>();
        assert_eq!(texture_sampler & (texture_sampler - 1), 0);

        let texture_view_dimension = std::mem::size_of::<TextureViewDimension>();
        assert_eq!(texture_view_dimension & (texture_view_dimension - 1), 0);

        let color_type = std::mem::size_of::<ColorType>();
        assert_eq!(color_type & (color_type - 1), 0);
    }
}
