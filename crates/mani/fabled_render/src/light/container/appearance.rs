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

// We are using TM-30 rather then using Black body for the chromaticity
// coordinates, since black body radiator enter the atmosphere, various gases
// absorb and alter the spectral content.
// TM-30 is a combination of Black body and Daylight to eliminates jump and
// attempts to smooth it out.

#[derive(Copy, Clone, Debug)]
pub struct ChromaticDesignation(pub [f32; 3]);

impl Default for ChromaticDesignation {
    fn default() -> Self {
        Self([0.31352, 0.32979, 0.35669])
    }
}


#[derive(Copy, Clone, Debug)]
pub struct LightAppearance {
    pub color: [f32; 3],
    pub chromaticity_coord: [f32; 3],
}

impl Default for LightAppearance {
    fn default() -> Self {
        Self {
            color: [1.0; 3],
            chromaticity_coord: [0.31352, 0.32979, 0.35669],
        }
    }
}

impl LightAppearance {
    pub const MATCH_FLAME: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.55511, 0.40638, 0.03851],
    };

    pub const CANDLE_FLAME: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.53891, 0.41095, 0.05014],
    };

    pub const SUN_SUNRISE_SUNSET: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.47699, 0.41368, 0.10933],
    };

    pub const HOUSEHOLD_TUNGSTEN: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.45986, 0.41060, 0.12954],
    };

    pub const TUNGSTEN_500W_1K: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.43693, 0.40407, 0.159],
    };

    pub const TUNGSTEN_2K: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.41860, 0.39694, 0.18446],
    };

    pub const TUNGSTEN_5K_10K: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.41221, 0.39406, 0.19373],
    };

    pub const QUARTZ: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.40530, 0.39072, 0.20398],
    };

    pub const FLUORESCENT: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.33596, 0.34955, 0.31449],
    };

    pub const SUN_DIRECT_NOON: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.33972, 0.35175, 0.30853],
    };

    pub const DAYLIGHT: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.32208, 0.33805, 0.33987],
    };

    pub const SKY_OVERCAST: LightAppearance = LightAppearance {
        color: [1.0; 3],
        chromaticity_coord: [0.30979, 0.32606, 0.36415],
    };
}

impl LightAppearance {
    pub fn new(chromaticity_coord: [f32; 2], color: [f32; 3]) -> Self {
        let [x, y] = chromaticity_coord;

        // value must be in a range of 0.0 to 1.0
        let safe_chromaticity_coord = [x.min(1.0), y.min(1.0)];


        let calculated_coord = [
            safe_chromaticity_coord[0],
            safe_chromaticity_coord[1],
            1.0 - safe_chromaticity_coord[0] - safe_chromaticity_coord[1],
        ];

        Self {
            chromaticity_coord: calculated_coord,
            color,
        }
    }
    // pub fn white_point(&self) -> [f32; 3] {
    // let maximum_luminance_scalar = 1.0 / self.chromaticity_coord[1];
    //
    // [
    // self.chromaticity_coord[0] * maximum_luminance_scalar,
    // self.chromaticity_coord[1] * maximum_luminance_scalar,
    // self.chromaticity_coord[2] * maximum_luminance_scalar,
    // ]
    // }
}
