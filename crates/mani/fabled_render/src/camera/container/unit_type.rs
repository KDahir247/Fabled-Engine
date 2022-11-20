use std::fmt::{Display, Formatter};

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

impl Display for ISOSpeedUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_repr = match self {
            ISOSpeedUnit::Arithmetic => "Arithmetic",
            ISOSpeedUnit::Logarithmic => "Logarithmic",
        };

        f.write_str(string_repr)
    }
}
