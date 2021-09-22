pub enum ISOSpeedUnit {
    Arithmetic,
    Logarithmic,
}

impl Default for ISOSpeedUnit {
    fn default() -> Self {
        Self::Arithmetic
    }
}
