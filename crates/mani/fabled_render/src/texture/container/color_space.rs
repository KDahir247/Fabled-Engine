#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorSpace {
    LinearSpace,
    GammaSpace,
}

impl From<ColorSpace> for basis_universal::ColorSpace {
    fn from(color_space: ColorSpace) -> Self {
        match color_space {
            ColorSpace::LinearSpace => basis_universal::ColorSpace::Linear,
            ColorSpace::GammaSpace => basis_universal::ColorSpace::Srgb,
        }
    }
}
