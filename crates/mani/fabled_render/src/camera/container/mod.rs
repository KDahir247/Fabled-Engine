mod camera_matrix;
mod fov_axis;
mod orientation;
mod orthographic;
mod perspective;
mod projection;
mod viewport;

pub use camera_matrix::*;
pub use fov_axis::*;
pub use orientation::*;
pub use orthographic::*;
pub use perspective::*;
pub use projection::*;
pub use viewport::*;

#[cfg(test)]
mod data_test {
    use crate::camera::{
        CameraMatrix, Orientation, DepthClamp, FovAxis, Orthographic, Perspective,
        PerspectiveDistance, PerspectiveOrientation, Projection, ProjectionCoordinate, ViewPort,
    };

    #[test]
    fn data_size() {
        let orthographic_size = std::mem::size_of::<Orthographic>();
        assert_eq!(orthographic_size & (orthographic_size - 1), 0);

        let perspective_size = std::mem::size_of::<Perspective>();
        assert_eq!(perspective_size & (perspective_size - 1), 0);

        let projection_size = std::mem::size_of::<Projection>();
        println!("{}", projection_size);

        let camera_orientation_size = std::mem::size_of::<Orientation>();
        println!("{}", camera_orientation_size);

        let camera_matrix_size = std::mem::size_of::<CameraMatrix>();
        assert_eq!(camera_matrix_size & (camera_matrix_size - 1), 0);

        let viewport_rect_size = std::mem::size_of::<ViewPort>();
        assert_eq!(viewport_rect_size & (viewport_rect_size - 1), 0);

        let fov_axis_size = std::mem::size_of::<FovAxis>();
        assert_eq!(fov_axis_size & (fov_axis_size - 1), 0);

        let perspective_orientation_size = std::mem::size_of::<PerspectiveOrientation>();
        assert_eq!(
            perspective_orientation_size & (perspective_orientation_size - 1),
            0
        );

        let perspective_distance_size = std::mem::size_of::<PerspectiveDistance>();
        assert_eq!(
            perspective_distance_size & (perspective_distance_size - 1),
            0
        );

        let projection_coordinate_size = std::mem::size_of::<ProjectionCoordinate>();
        assert_eq!(
            projection_coordinate_size & (projection_coordinate_size - 1),
            0
        );

        let depth_clamp_size = std::mem::size_of::<DepthClamp>();
        assert_eq!(depth_clamp_size & (depth_clamp_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let orthographic_alignment = std::mem::align_of::<Orthographic>();
        assert_eq!(orthographic_alignment & (orthographic_alignment - 1), 0);

        let perspective_alignment = std::mem::align_of::<Perspective>();
        assert_eq!(perspective_alignment & (perspective_alignment - 1), 0);

        let projection_alignment = std::mem::align_of::<Projection>();
        assert_eq!(projection_alignment & (projection_alignment - 1), 0);

        let camera_orientation_alignment = std::mem::align_of::<Orientation>();
        assert_eq!(
            camera_orientation_alignment & (camera_orientation_alignment - 1),
            0
        );

        let camera_matrix_alignment = std::mem::align_of::<CameraMatrix>();
        println!("{}", camera_matrix_alignment);

        let viewport_alignment = std::mem::align_of::<ViewPort>();
        println!("{}", viewport_alignment);

        let fov_axis_alignment = std::mem::align_of::<FovAxis>();
        print!("{}", fov_axis_alignment);

        let perspective_orientation_alignment = std::mem::align_of::<PerspectiveOrientation>();
        assert_eq!(
            perspective_orientation_alignment & (perspective_orientation_alignment - 1),
            0
        );

        let perspective_distance_alignment = std::mem::align_of::<PerspectiveDistance>();
        assert_eq!(
            perspective_distance_alignment & (perspective_distance_alignment - 1),
            0
        );

        let projection_coordinates_alignment = std::mem::align_of::<ProjectionCoordinate>();
        assert_eq!(
            projection_coordinates_alignment & (projection_coordinates_alignment - 1),
            0
        );

        let depth_clamp_alignment = std::mem::align_of::<DepthClamp>();
        assert_eq!(depth_clamp_alignment & (depth_clamp_alignment - 1), 0);
    }
}
