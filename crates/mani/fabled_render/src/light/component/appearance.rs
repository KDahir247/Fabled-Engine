use crate::color::{cct_to_illuminant_d, cct_to_linear};
use fabled_math::{Vector3, Vector4};

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
// ---------------------------------------------
#[derive(Copy, Clone)]
pub struct LightAppearance {
    // Stores Color in xyz and temperature in w
    pub appearance: Vector4,
}


impl LightAppearance {
    pub const MATCH_FLAME: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 1700.0),
    };

    pub const CANDLE_FLAME: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 1850.0),
    };

    pub const SUN_SUNRISE_SUNSET: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 2000.0),
    };

    pub const HOUSEHOLD_TUNGSTEN: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 2500.0),
    };

    pub const TUNGSTEN_500W_1K: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 3000.0),
    };

    pub const TUNGSTEN_2K: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 3200.0),
    };

    pub const TUNGSTEN_5K_10K: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 4500.0),
    };

    pub const QUARTZ: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 3200.0),
    };

    pub const FLUORESCENT: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 4000.0),
    };

    pub const SUN_DIRECT_NOON: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 5000.0),
    };

    pub const DAYLIGHT: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 6000.0),
    };

    pub const SKY_OVERCAST: LightAppearance = LightAppearance {
        appearance: Vector4::set(1.0, 1.0, 1.0, 7000.0),
    };
}

impl LightAppearance {
    pub fn new(color: Vector3, temperature: f32) -> Self {
        LightAppearance {
            appearance: Vector4::set(color.x(), color.y(), color.z(), temperature),
        }
    }

    pub fn compute_color_temperature_d(self) -> Vector3 {
        cct_to_illuminant_d(self.appearance.w())
    }

    pub fn compute_color_temperature(self) -> Vector3 {
        cct_to_linear(self.appearance.w())
    }
}
