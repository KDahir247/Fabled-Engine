mod any_loader;
mod aspect_ratio_loader;
mod camera_format_loader;
mod camera_projection_loader;
mod decay_loader;
mod v7400_camera_loader;
mod v7400_light_loader;
mod v7400_loader;

pub use any_loader::*;
pub(crate) use aspect_ratio_loader::*;
pub(crate) use camera_format_loader::*;
pub(crate) use camera_projection_loader::*;
pub(crate) use decay_loader::*;
pub use v7400_loader::*;

#[cfg(test)]
mod data_test {
    use crate::V7400Loader;

    #[test]
    fn data_size() {
        let v7400_loader = std::mem::size_of::<V7400Loader>();
        println!("{:?}", v7400_loader);
    }

    #[test]
    fn data_alignment() {}
}
