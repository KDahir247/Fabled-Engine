pub use aperture_format::*;
pub use aperture_mode::*;
pub use aspect_ratio::*;
pub use aspect_ratio_mode::*;
pub use camera_format::*;
pub use clipping_plane::*;
pub use f_stop::*;
pub use fish_eye_len::*;
pub use fov::*;
pub use fov_scaling::*;
pub use gate_fit::*;
pub use iso_speed::*;
pub use oblique::*;
pub use orthographic::*;
pub use perspective::*;
pub use projection::*;
pub use shutter::*;
pub use unit_type::*;
pub use viewport::*;

mod aperture_format;
mod aperture_mode;
mod aspect_ratio;
mod aspect_ratio_mode;
mod camera_format;
mod clipping_plane;
mod f_stop;
mod fish_eye_len;
mod fov;
mod fov_scaling;
mod gate_fit;
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
        Aperture, ApertureMode, AspectRatio, AspectRatioMode, CameraFormat, ClippingPlane, FStop,
        FishLens, Fov, FovAxis, FovScalingAlgorithm, GateFit, ISOSpeed, ISOSpeedUnit, Oblique,
        Orthographic, Perspective, Projection, Shutter, ViewPort,
    };

    #[test]
    fn data_size() {
        let orthographic_size = std::mem::size_of::<Orthographic>();
        assert_eq!(orthographic_size & (orthographic_size - 1), 0);

        let perspective_size = std::mem::size_of::<Perspective>();
        assert_eq!(perspective_size & (perspective_size - 1), 0);

        let projection_size = std::mem::size_of::<Projection>();
        assert_eq!(projection_size & (projection_size - 1), 0);

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

        let aperture_len_size = std::mem::size_of::<FStop>();
        assert_eq!(aperture_len_size & (aperture_len_size - 1), 0);

        let iso_speed_size = std::mem::size_of::<ISOSpeed>();
        assert_eq!(iso_speed_size & (iso_speed_size - 1), 0);

        let iso_speed_unit_size = std::mem::size_of::<ISOSpeedUnit>();
        assert_eq!(iso_speed_unit_size & (iso_speed_unit_size - 1), 0);

        let oblique_size = std::mem::size_of::<Oblique>();
        assert_eq!(oblique_size & (oblique_size - 1), 0);

        let shutter_size = std::mem::size_of::<Shutter>();
        assert_eq!(shutter_size & (shutter_size - 1), 0);

        let camera_format_size = std::mem::size_of::<CameraFormat>();
        assert_eq!(camera_format_size & (camera_format_size - 1), 0);

        let fov_algorithm_size = std::mem::size_of::<FovScalingAlgorithm>();
        assert_eq!(fov_algorithm_size & (fov_algorithm_size - 1), 0);

        let aperture_format_size = std::mem::size_of::<Aperture>();
        assert_eq!(aperture_format_size & (aperture_format_size - 1), 0);

        let aperture_mode_size = std::mem::size_of::<ApertureMode>();
        assert_eq!(aperture_mode_size & (aperture_mode_size - 1), 0);

        let aspect_ratio_mode_size = std::mem::size_of::<AspectRatioMode>();
        assert_eq!(aspect_ratio_mode_size & (aspect_ratio_mode_size - 1), 0);

        let gate_fit_size = std::mem::size_of::<GateFit>();
        assert_eq!(gate_fit_size & (gate_fit_size - 1), 0);

        let fish_len_size = std::mem::size_of::<FishLens>();
        assert_eq!(fish_len_size & (fish_len_size - 1), 0);
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

        let aperture_len_alignment = std::mem::align_of::<FStop>();
        assert_eq!(aperture_len_alignment & (aperture_len_alignment - 1), 0);

        let iso_speed_alignment = std::mem::align_of::<ISOSpeed>();
        assert_eq!(iso_speed_alignment & (iso_speed_alignment - 1), 0);

        let iso_speed_unit_alignment = std::mem::align_of::<ISOSpeedUnit>();
        assert_eq!(iso_speed_unit_alignment & (iso_speed_unit_alignment - 1), 0);

        let oblique_alignment = std::mem::align_of::<Oblique>();
        assert_eq!(oblique_alignment & (oblique_alignment - 1), 0);

        let shutter_alignment = std::mem::align_of::<Shutter>();
        assert_eq!(shutter_alignment & (shutter_alignment - 1), 0);

        let camera_format_alignment = std::mem::align_of::<CameraFormat>();
        assert_eq!(camera_format_alignment & (camera_format_alignment - 1), 0);

        let fov_algorithm_alignment = std::mem::align_of::<FovScalingAlgorithm>();
        assert_eq!(fov_algorithm_alignment & (fov_algorithm_alignment - 1), 0);

        let aperture_format_alignment = std::mem::align_of::<Aperture>();
        assert_eq!(
            aperture_format_alignment & (aperture_format_alignment - 1),
            0
        );

        let aperture_mode_alignment = std::mem::align_of::<ApertureMode>();
        assert_eq!(aperture_mode_alignment & (aperture_mode_alignment - 1), 0);

        let aspect_ratio_mode_alignment = std::mem::align_of::<AspectRatioMode>();
        assert_eq!(
            aspect_ratio_mode_alignment & (aspect_ratio_mode_alignment - 1),
            0
        );

        let gate_fit_alignment = std::mem::align_of::<GateFit>();
        assert_eq!(gate_fit_alignment & (gate_fit_alignment - 1), 0);

        let fish_len_alignment = std::mem::align_of::<FishLens>();
        assert_eq!(fish_len_alignment & (fish_len_alignment - 1), 0);
    }
}
