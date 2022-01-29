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

#[derive(Copy, Clone, Debug)]
pub struct LightAppearance {
    pub color: [f32; 3],
    pub temperature: f32,
}

impl Default for LightAppearance {
    fn default() -> Self {
        Self {
            temperature: 6500.0,
            color: [1.0; 3],
        }
    }
}

impl LightAppearance {
    pub const MATCH_FLAME: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 1750.0,
    };

    pub const CANDLE_FLAME: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 1890.0,
    };

    pub const SUN_SUNRISE_SUNSET: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 2500.0,
    };

    pub const HOUSEHOLD_TUNGSTEN: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 2700.0,
    };

    pub const TUNGSTEN_500W_1K: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 3000.0,
    };

    pub const TUNGSTEN_2K: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 3275.0,
    };

    pub const TUNGSTEN_5K_10K: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 3380.0,
    };

    pub const QUARTZ: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 3500.0,
    };

    pub const FLUORESCENT: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 5350.0,
    };

    pub const SUN_DIRECT_NOON: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 5200.0,
    };

    pub const DAYLIGHT: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 6000.0,
    };

    pub const SKY_OVERCAST: LightAppearance = LightAppearance {
        color: [1.0; 3],
        temperature: 6750.0,
    };
}

impl LightAppearance {
    pub fn new(kelvin: f32, temperature_unit_type: TemperatureUnit, color: [f32; 3]) -> Self {
        // Convert unit type to Kelvin.
        let temperature = match temperature_unit_type {
            TemperatureUnit::Kelvin => kelvin,
            TemperatureUnit::Celsius => celsius_to_kelvin(kelvin),
            TemperatureUnit::Fahrenheit => fahrenheit_to_kelvin(kelvin),
        };

        Self { temperature, color }
    }
}
