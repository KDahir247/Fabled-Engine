use crate::light::{celsius_to_kelvin, fahrenheit_to_kelvin};
use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum TemperatureType {
    Celsius,
    Fahrenheit,
    Kelvin,
}


#[derive(Copy, Clone, PartialEq)]
pub struct Temperature {
    pub kelvin: f32,
}

impl Temperature {
    pub fn new(value: f32, ty: TemperatureType) -> Temperature {
        let kelvin = match ty {
            TemperatureType::Celsius => celsius_to_kelvin(value),
            TemperatureType::Fahrenheit => fahrenheit_to_kelvin(value),
            TemperatureType::Kelvin => value,
        };

        Temperature { kelvin }
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temperature(Kelvin : {})", self.kelvin)
    }
}

// When temperature change we will do adaption of previous temp and current
// temp.
impl Component for Temperature {
    type Tracking = Modification;
}
