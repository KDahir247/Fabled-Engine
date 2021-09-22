use crate::camera::ISOSpeed;

#[derive(Copy, Clone, Debug)]
pub enum LightUnit {
    Lumen,
    Candela,
    Lux,
    EV100 {
        iso: ISOSpeed,
        calibration_constant: Option<f32>,
    },
}


impl Default for LightUnit {
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
