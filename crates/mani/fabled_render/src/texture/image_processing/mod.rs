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
mod data_alignment_test {
    use crate::texture::image_processing::{FilterType, ImageProcessing};

    #[test]
    fn data_alignment() {
        let filter_type = std::mem::size_of::<FilterType>();
        assert_eq!(filter_type & (filter_type - 1), 0);

        let img_proc = std::mem::size_of::<ImageProcessing>();
        assert_eq!(img_proc & (img_proc - 1), 0);
    }
}
