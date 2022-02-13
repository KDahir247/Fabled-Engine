mod affine_3d;
mod mat3;
mod mat4;

pub use affine_3d::*;
pub use mat3::*;
pub use mat4::*;

#[cfg(test)]
mod data_test {
    use crate::Affine3;

    #[test]
    fn data_size() {
        let affine_3d_size = std::mem::size_of::<Affine3>();
        println!("{}", affine_3d_size);
    }


    #[test]
    fn data_alignment() {
        let affine_3d_alignment = std::mem::align_of::<Affine3>();
        println!("{}", affine_3d_alignment);
    }
}
