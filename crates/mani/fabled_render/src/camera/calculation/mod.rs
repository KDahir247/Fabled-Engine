mod camera;

pub use camera::*;


#[cfg(test)]
mod data_test {
    use crate::camera::Camera;

    #[test]
    fn data_size() {
        let camera_matrix_size = std::mem::size_of::<Camera>();
        assert_eq!(camera_matrix_size & (camera_matrix_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let camera_matrix_alignment = std::mem::align_of::<Camera>();
        assert_eq!(camera_matrix_alignment & (camera_matrix_alignment - 1), 0);
    }
}
