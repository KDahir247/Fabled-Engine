use crate::camera::{logarithmic_to_arithmetic_speed, ISOSpeedUnit, ISO_ARITHMETIC_STANDARD};
use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};
// The higher the film speed number (ISO Speed), the greater the film
// sensitivity to light. Having a low film speed number will result in the film
// not being sensitivity sensitive to light.

// Having a high film speed number (ISO Speed) will also increase the noise.
// The higher the film speed number the more noise the image will have.
// While having a low film speed number will have lower noise to the image.

#[derive(Copy, Clone, PartialEq)]
pub struct ISOSpeed {
    pub arithmetic_speed: f32,
}

impl Default for ISOSpeed {
    fn default() -> ISOSpeed {
        ISOSpeed {
            arithmetic_speed: 100.0,
        }
    }
}

impl ISOSpeed {
    pub fn new(iso_speed: f32, unit_type: ISOSpeedUnit) -> ISOSpeed {
        let mut iso_speed = iso_speed;

        if let ISOSpeedUnit::Logarithmic = unit_type {
            iso_speed = logarithmic_to_arithmetic_speed(iso_speed);
        };

        ISOSpeed {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn new_standard(iso_speed: f32, unit_type: ISOSpeedUnit) -> ISOSpeed {
        let mut iso_speed = iso_speed;


        if let ISOSpeedUnit::Logarithmic = unit_type {
            iso_speed = logarithmic_to_arithmetic_speed(iso_speed);
        }

        if let false = ISO_ARITHMETIC_STANDARD.contains(&iso_speed) {
            iso_speed = 100.0
        };

        ISOSpeed {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn is_standard(&self) -> bool {
        ISO_ARITHMETIC_STANDARD.contains(&self.arithmetic_speed)
    }
}

impl Component for ISOSpeed {
    type Tracking = Modification;
}

impl Display for ISOSpeed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISO(arithmetic speed : {})", self.arithmetic_speed)
    }
}
