mod orientation;
mod orthographic;
mod perspective;
mod projection;

pub use orientation::*;
pub use orthographic::*;
pub use perspective::*;
pub use projection::*;

#[cfg(test)]
mod data_test {
    use crate::camera::{CameraOrientation, Orthographic, Perspective, Projection};

    #[test]
    fn data_size() {
        let orthographic_size = std::mem::size_of::<Orthographic>();
        assert_eq!(orthographic_size & (orthographic_size - 1), 0);

        let perspective_size = std::mem::size_of::<Perspective>();
        assert_eq!(perspective_size & (perspective_size - 1), 0);

        let projection_size = std::mem::size_of::<Projection>();
        println!("{}", projection_size);

        let camera_orientation_size = std::mem::size_of::<CameraOrientation>();
        println!("{}", camera_orientation_size);
    }

    #[test]
    fn data_alignment() {
        let orthographic_alignment = std::mem::align_of::<Orthographic>();
        assert_eq!(orthographic_alignment & (orthographic_alignment - 1), 0);

        let perspective_alignment = std::mem::align_of::<Perspective>();
        assert_eq!(perspective_alignment & (perspective_alignment - 1), 0);

        let projection_alignment = std::mem::align_of::<Projection>();
        assert_eq!(projection_alignment & (projection_alignment - 1), 0);

        let camera_orientation_alignment = std::mem::align_of::<CameraOrientation>();
        assert_eq!(
            camera_orientation_alignment & (camera_orientation_alignment - 1),
            0
        );
    }
}
