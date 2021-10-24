mod border_color;
mod color_space;
mod color_type;
mod extent2d;
mod extent3d;
mod flip_axis;
mod texture;
mod texture_access;
mod texture_aspect;
mod texture_data;
mod texture_dimension;
mod texture_sample_type;
mod texture_sampler;
mod texture_view_dimension;

pub use border_color::*;
pub use color_space::*;
pub use color_type::*;
pub use extent2d::*;
pub use extent3d::*;
pub use flip_axis::*;
pub use texture::*;
pub use texture_access::*;
pub use texture_aspect::*;
pub use texture_data::*;
pub use texture_dimension::*;
pub use texture_sample_type::*;
pub use texture_sampler::*;
pub use texture_view_dimension::*;

#[cfg(test)]
pub mod data_test {
    use crate::texture::container::*;

    #[test]
    fn data_size() {
        let border_color_size = std::mem::size_of::<BorderColor>();
        assert_eq!(border_color_size & (border_color_size - 1), 0);

        let color_space_size = std::mem::size_of::<ColorSpace>();
        assert_eq!(color_space_size & (color_space_size - 1), 0);

        let extent3d_size = std::mem::size_of::<Extent3d>();
        assert_eq!(extent3d_size & (extent3d_size - 1), 0);

        let extend2d_size = std::mem::size_of::<Extent2d>();
        assert_eq!(extend2d_size & (extend2d_size - 1), 0);

        let flip_axis_size = std::mem::size_of::<FlipAxis>();
        assert_eq!(flip_axis_size & (flip_axis_size - 1), 0);

        let texture_access_size = std::mem::size_of::<StorageTextureAccess>();
        assert_eq!(texture_access_size & (texture_access_size - 1), 0);

        let texture_aspect_size = std::mem::size_of::<TextureAspect>();
        assert_eq!(texture_aspect_size & (texture_aspect_size - 1), 0);

        let texture_data_size = std::mem::size_of::<TextureData>();
        assert_eq!(texture_data_size & (texture_data_size - 1), 0);

        let texture_dimension_size = std::mem::size_of::<TextureDimension>();
        assert_eq!(texture_dimension_size & (texture_dimension_size - 1), 0);

        let texture_sample_type_size = std::mem::size_of::<TextureSampleType>();
        assert_eq!(texture_sample_type_size & (texture_sample_type_size - 1), 0);

        let texture_sampler_size = std::mem::size_of::<TextureSampler>();
        assert_eq!(texture_sampler_size & (texture_sampler_size - 1), 0);

        let texture_view_dimension_size = std::mem::size_of::<TextureViewDimension>();
        assert_eq!(
            texture_view_dimension_size & (texture_view_dimension_size - 1),
            0
        );

        let color_type_size = std::mem::size_of::<ColorType>();
        assert_eq!(color_type_size & (color_type_size - 1), 0);

        let color_target_size = std::mem::size_of::<ColorTarget>();
        assert_eq!(color_target_size & (color_target_size - 1), 0);

        let standard_texture_size = std::mem::size_of::<Texture>();
        assert_eq!(standard_texture_size & (standard_texture_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let border_color_alignment = std::mem::align_of::<BorderColor>();
        assert_eq!(border_color_alignment & (border_color_alignment - 1), 0);

        let color_space_alignment = std::mem::align_of::<ColorSpace>();
        assert_eq!(color_space_alignment & (color_space_alignment - 1), 0);

        let extent3d_alignment = std::mem::align_of::<Extent3d>();
        assert_eq!(extent3d_alignment & (extent3d_alignment - 1), 0);

        let extend2d_alignment = std::mem::align_of::<Extent2d>();
        assert_eq!(extend2d_alignment & (extend2d_alignment - 1), 0);

        let flip_axis_alignment = std::mem::align_of::<FlipAxis>();
        assert_eq!(flip_axis_alignment & (flip_axis_alignment - 1), 0);

        let texture_access_alignment = std::mem::align_of::<StorageTextureAccess>();
        assert_eq!(texture_access_alignment & (texture_access_alignment - 1), 0);

        let texture_aspect_alignment = std::mem::align_of::<TextureAspect>();
        assert_eq!(texture_aspect_alignment & (texture_aspect_alignment - 1), 0);

        let texture_data_alignment = std::mem::align_of::<TextureData>();
        assert_eq!(texture_data_alignment & (texture_data_alignment - 1), 0);

        let texture_dimension_alignment = std::mem::align_of::<TextureDimension>();
        assert_eq!(
            texture_dimension_alignment & (texture_dimension_alignment - 1),
            0
        );

        let texture_sample_type_alignment = std::mem::align_of::<TextureSampleType>();
        assert_eq!(
            texture_sample_type_alignment & (texture_sample_type_alignment - 1),
            0
        );

        let texture_sampler_alignment = std::mem::align_of::<TextureSampler>();
        assert_eq!(
            texture_sampler_alignment & (texture_sampler_alignment - 1),
            0
        );

        let texture_view_dimension_alignment = std::mem::align_of::<TextureViewDimension>();
        assert_eq!(
            texture_view_dimension_alignment & (texture_view_dimension_alignment - 1),
            0
        );

        let color_type_alignment = std::mem::align_of::<ColorType>();
        assert_eq!(color_type_alignment & (color_type_alignment - 1), 0);

        let color_target_alignment = std::mem::align_of::<ColorTarget>();
        assert_eq!(color_target_alignment & (color_target_alignment - 1), 0);

        let standard_texture_alignment = std::mem::align_of::<Texture>();
        assert_eq!(
            standard_texture_alignment & (standard_texture_alignment - 1),
            0
        );
    }
}
