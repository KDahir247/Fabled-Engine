#[derive(Copy, Clone)]
pub enum ISOSpeedUnit {
    Arithmetic,
    Logarithmic,
}

impl Default for ISOSpeedUnit {
    fn default() -> Self {
        Self::Arithmetic
    }
}
