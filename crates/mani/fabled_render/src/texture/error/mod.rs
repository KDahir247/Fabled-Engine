pub mod codecs_error;
mod image_proc_error;
mod ktx_error;
mod texture_error;

pub use codecs_error::*;
pub use image_proc_error::*;
pub use ktx_error::*;
pub use texture_error::*;

#[cfg(test)]
mod data_test {
    use crate::texture::FlipType;

    #[test]
    fn data_size() {
        let flip_type_size = std::mem::size_of::<FlipType>();
        assert_eq!(flip_type_size & (flip_type_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let flip_type_alignment = std::mem::align_of::<FlipType>();
        assert_eq!(flip_type_alignment & (flip_type_alignment - 1), 0);
    }
}
