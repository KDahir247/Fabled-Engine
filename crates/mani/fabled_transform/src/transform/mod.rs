mod orientation;
mod transform;

pub use orientation::*;
pub use transform::*;


#[cfg(test)]
mod data_test {
    use crate::transform::transform::Transform;
    use crate::Orientation;

    #[test]
    fn data_size() {
        let transform_size = std::mem::size_of::<Transform>();
        println!("transform size {}", transform_size);

        let orientation_size = std::mem::size_of::<Orientation>();
        assert_eq!(orientation_size & (orientation_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let transform_alignment = std::mem::align_of::<Transform>();
        assert_eq!(transform_alignment & (transform_alignment - 1), 0);

        let orientation_alignment = std::mem::align_of::<Orientation>();
        assert_eq!(orientation_alignment & (orientation_alignment - 1), 0);
    }
}
