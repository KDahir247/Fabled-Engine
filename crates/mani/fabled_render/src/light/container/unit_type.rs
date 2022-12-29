use crate::camera::ISOSpeed;

#[derive(Copy, Clone, PartialEq)]
pub enum IntensityUnit {
    Lumen,
    Candela,
    Lux {
        distance: f32,
    },
    EV100 {
        iso: ISOSpeed,
        calibration_constant: Option<f32>,
    },
}


impl Default for IntensityUnit {
    fn default() -> Self {
        Self::Lumen
    }
}


#[derive(Copy, Clone, Debug)]
pub enum TemperatureUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        Self::Kelvin
    }
}
