// | Degree Kelvin | Type of Light Source      |
// |---------------|---------------------------|
// | 1700-1800K    | Match flame               |
// | 1850-1930K    | Candle flame              |
// | 2000-3000K    | Sun at sunrise or sunset  |
// | 2500-2900k    | Household tungsten bulb   |
// | 3000K         | Tungsten lamp 500W-1k     |
// | 3200-3500K    | Quartz lights             |
// | 3200-7500K    | Fluorescent lights        |
// | 3275K         | Tungsten lamp 2K          |
// | 3380K         | Tungsten lamp 5K, 10K     |
// | 5000-5400K    | Sun direct at noon        |
// | 5500-6500K    | Daylight (Sun + Sky)      |
// | 5500-6500K    | Sun through clouds/haze   |
// | 6000-7500K    | Sky overcast              |
// | 6500K         | RGB monitor (white point) |
// | 7000-8000K    | Outdoor shade areas       |
// | 8000-10000K   | Sky partly cloudy         |
// -_-------------------------------------------


use crate::light::{celsius_to_kelvin, fahrenheit_to_kelvin, TemperatureUnit};

// temperature is in kelvin
#[derive(Copy, Clone, Debug)]
pub struct LightAppearance {
    pub color: [f32; 3],
    pub temperature: f32,
}

impl Default for LightAppearance {
    fn default() -> Self {
        Self {
            temperature: 6500.0,
            color: [0.0; 3],
        }
    }
}

impl LightAppearance {
    pub fn new(unit: f32, unit_type: TemperatureUnit, color: [f32; 3]) -> Self {
        let mut unit = unit;

        // Convert unit type to Kelvin.
        match unit_type {
            TemperatureUnit::Kelvin => {} // Already in kelvin, so no conversion needed.
            TemperatureUnit::Celsius => {
                unit = celsius_to_kelvin(unit);
            }
            TemperatureUnit::Fahrenheit => {
                unit = fahrenheit_to_kelvin(unit);
            }
        }

        Self {
            temperature: unit,
            color,
        }
    }
}
