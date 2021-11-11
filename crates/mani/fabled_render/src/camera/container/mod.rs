pub use aperture_len::*;
pub use aspect_ratio::*;
pub use clipping_plane::*;
pub use fov::*;
pub use iso_speed::*;
pub use oblique::*;
pub use orthographic::*;
pub use perspective::*;
pub use projection::*;
pub use shutter::*;
pub use unit_type::*;
pub use viewport::*;

mod aperture_len;
mod aspect_ratio;
mod clipping_plane;
mod fov;
mod iso_speed;
mod oblique;
mod orthographic;
mod perspective;
mod projection;
mod shutter;
mod unit_type;
mod viewport;


#[cfg(test)]
mod data_test {
    use crate::camera::{
        AspectRatio, ClippingPlane, Fov, FovAxis, FullStop, ISOSpeed, Oblique, Orthographic,
        Perspective, Projection, Shutter, ViewPort,
    };

    #[test]
    fn data_size() {
        let orthographic_size = std::mem::size_of::<Orthographic>();
        assert_eq!(orthographic_size & (orthographic_size - 1), 0);

        let perspective_size = std::mem::size_of::<Perspective>();
        assert_eq!(perspective_size & (perspective_size - 1), 0);

        let projection_size = std::mem::size_of::<Projection>();
        println!("Projection size {}", projection_size);

        let viewport_rect_size = std::mem::size_of::<ViewPort>();
        assert_eq!(viewport_rect_size & (viewport_rect_size - 1), 0);

        let fov_axis_size = std::mem::size_of::<FovAxis>();
        assert_eq!(fov_axis_size & (fov_axis_size - 1), 0);

        let fov_size = std::mem::size_of::<Fov>();
        assert_eq!(fov_size & (fov_size - 1), 0);

        let clipping_plane_size = std::mem::size_of::<ClippingPlane>();
        assert_eq!(clipping_plane_size & (clipping_plane_size - 1), 0);

        let aspect_ratio_size = std::mem::size_of::<AspectRatio>();
        assert_eq!(aspect_ratio_size & (aspect_ratio_size - 1), 0);

        let aperture_len_size = std::mem::size_of::<FullStop>();
        assert_eq!(aperture_len_size & (aperture_len_size - 1), 0);

        let iso_speed_size = std::mem::size_of::<ISOSpeed>();
        assert_eq!(iso_speed_size & (iso_speed_size - 1), 0);

        let oblique_size = std::mem::size_of::<Oblique>();
        assert_eq!(oblique_size & (oblique_size - 1), 0);

        let shutter_size = std::mem::size_of::<Shutter>();
        assert_eq!(shutter_size & (shutter_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let orthographic_alignment = std::mem::align_of::<Orthographic>();
        assert_eq!(orthographic_alignment & (orthographic_alignment - 1), 0);


        let perspective_alignment = std::mem::align_of::<Perspective>();
        assert_eq!(perspective_alignment & (perspective_alignment - 1), 0);


        let projection_alignment = std::mem::align_of::<Projection>();
        assert_eq!(projection_alignment & (projection_alignment - 1), 0);

        let viewport_alignment = std::mem::align_of::<ViewPort>();
        assert_eq!(viewport_alignment & (viewport_alignment - 1), 0);

        let fov_axis_alignment = std::mem::align_of::<FovAxis>();
        assert_eq!(fov_axis_alignment & (fov_axis_alignment - 1), 0);

        let fov_alignment_alignment = std::mem::align_of::<Fov>();
        assert_eq!(fov_alignment_alignment & (fov_alignment_alignment - 1), 0);

        let clipping_plane_alignment = std::mem::align_of::<ClippingPlane>();
        assert_eq!(clipping_plane_alignment & (clipping_plane_alignment - 1), 0);

        let aspect_ratio_alignment = std::mem::align_of::<AspectRatio>();
        assert_eq!(aspect_ratio_alignment & (aspect_ratio_alignment - 1), 0);

        let aperture_len_alignment = std::mem::align_of::<FullStop>();
        assert_eq!(aperture_len_alignment & (aperture_len_alignment - 1), 0);

        let iso_speed_alignment = std::mem::align_of::<ISOSpeed>();
        assert_eq!(iso_speed_alignment & (iso_speed_alignment - 1), 0);

        let oblique_alignment = std::mem::align_of::<Oblique>();
        assert_eq!(oblique_alignment & (oblique_alignment - 1), 0);

        let shutter_alignment = std::mem::align_of::<Shutter>();
        assert_eq!(shutter_alignment & (shutter_alignment - 1), 0);
    }
}
