use crate::light::ISOSpeed;

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
