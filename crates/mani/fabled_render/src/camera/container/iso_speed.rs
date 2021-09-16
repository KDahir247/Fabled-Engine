use crate::camera::{logarithmic_to_arithmetic_speed, ISOSpeedUnit, ISO_ARITHMETIC_STANDARD};

#[derive(Copy, Clone, Debug)]
pub struct ISOSpeed {
    pub arithmetic_speed: f32,
}

impl Default for ISOSpeed {
    fn default() -> Self {
        Self {
            arithmetic_speed: 100.0,
        }
    }
}

impl ISOSpeed {
    pub fn new(iso_speed: f32, unit_type: ISOSpeedUnit) -> Self {
        let mut iso_speed = iso_speed;

        if let ISOSpeedUnit::Logarithmic = unit_type {
            iso_speed = logarithmic_to_arithmetic_speed(iso_speed);
        };

        Self {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn new_standard(iso_speed: f32, unit_type: ISOSpeedUnit) -> Self {
        let mut iso_speed = iso_speed;


        if let ISOSpeedUnit::Logarithmic = unit_type {
            iso_speed = logarithmic_to_arithmetic_speed(iso_speed);
        }

        if let false = ISO_ARITHMETIC_STANDARD.contains(&iso_speed) {
            iso_speed = 100.0
        };

        Self {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn is_standard(&self) -> bool {
        ISO_ARITHMETIC_STANDARD.contains(&self.arithmetic_speed)
    }
}
