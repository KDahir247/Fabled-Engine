mod image_proc;
pub use image_proc::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl From<FilterType> for image::imageops::FilterType {
    fn from(filter: FilterType) -> Self {
        match filter {
            FilterType::Nearest => image::imageops::FilterType::Nearest,
            FilterType::Triangle => image::imageops::FilterType::Triangle,
            FilterType::CatmullRom => image::imageops::FilterType::CatmullRom,
            FilterType::Gaussian => image::imageops::FilterType::Gaussian,
            FilterType::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

#[cfg(test)]
mod data_test {
    use crate::texture::image_processing::{FilterType, ImageProcessing};

    #[test]
    fn data_size() {
        let filter_type_size = std::mem::size_of::<FilterType>();
        assert_eq!(filter_type_size & (filter_type_size - 1), 0);

        let img_proc_size = std::mem::size_of::<ImageProcessing>();
        assert_eq!(img_proc_size & (img_proc_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let filter_type_alignment = std::mem::align_of::<FilterType>();
        assert_eq!(filter_type_alignment & (filter_type_alignment - 1), 0);

        let img_proc_alignment = std::mem::align_of::<ImageProcessing>();
        assert_eq!(img_proc_alignment & (img_proc_alignment - 1), 0);
    }
}
