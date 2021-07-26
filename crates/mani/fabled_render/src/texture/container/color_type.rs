#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorType {
    L8,

    La8,

    Rgba8,

    L16,

    La16,

    Rgb8,

    Rgb16,

    Rgba16,

    Bgr8,

    Bgra8,

    Nil,
}

impl From<image::ColorType> for ColorType {
    fn from(color_type: image::ColorType) -> Self {
        match color_type {
            image::ColorType::L8 => ColorType::L8,
            image::ColorType::La8 => ColorType::La8,
            image::ColorType::Rgba8 => ColorType::Rgba8,
            image::ColorType::L16 => ColorType::L16,
            image::ColorType::La16 => ColorType::La16,
            image::ColorType::Rgb16 => ColorType::Rgb16,
            image::ColorType::Rgba16 => ColorType::Rgba16,
            image::ColorType::Bgr8 => ColorType::Bgr8,
            image::ColorType::Bgra8 => ColorType::Bgra8,
            image::ColorType::Rgb8 => ColorType::Rgb8,
            _ => ColorType::Nil,
        }
    }
}
