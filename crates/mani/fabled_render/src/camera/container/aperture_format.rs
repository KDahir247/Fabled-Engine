// Camera aperture (sensor size in unity)
// size of camera sensor in millimeters

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Default)]
pub struct Aperture {
    pub aperture_size_x: f32,
    pub aperture_size_y: f32,
}
impl Aperture {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            aperture_size_x: width,
            aperture_size_y: height,
        }
    }

    pub const R8MM: Aperture = Aperture {
        aperture_size_x: 4.8,
        aperture_size_y: 3.5,
    };
    pub const R8MM_SUPER: Aperture = Aperture {
        aperture_size_x: 5.79,
        aperture_size_y: 4.01,
    };
    pub const R16MM_THEATRICAL: Aperture = Aperture {
        aperture_size_x: 10.26,
        aperture_size_y: 7.49,
    };
    pub const SUPER_16MM: Aperture = Aperture {
        aperture_size_x: 12.52,
        aperture_size_y: 7.41,
    };
    pub const ULTRA_16MM: Aperture = Aperture {
        aperture_size_x: 11.66,
        aperture_size_y: 7.49,
    };
    pub const R35MM_ACADEMY: Aperture = Aperture {
        aperture_size_x: 21.0,
        aperture_size_y: 15.2,
    };
    pub const R35MM_2PERF: Aperture = Aperture {
        aperture_size_x: 21.95,
        aperture_size_y: 9.35,
    };
    pub const R35MM_TECHNISCOPE: Aperture = Aperture {
        aperture_size_x: 22.0,
        aperture_size_y: 9.47,
    };
    pub const R35MM_185_PROJECTION: Aperture = Aperture {
        aperture_size_x: 22.0,
        aperture_size_y: 16.0,
    };
    pub const R35MM_ANAMORPHIC: Aperture = Aperture {
        aperture_size_x: 21.95,
        aperture_size_y: 18.6,
    };
    pub const SUPER_35MM: Aperture = Aperture {
        aperture_size_x: 24.89,
        aperture_size_y: 18.66,
    };
    pub const R35_ALEXA: Aperture = Aperture {
        aperture_size_x: 54.12,
        aperture_size_y: 25.59,
    };
    pub const R70MM: Aperture = Aperture {
        aperture_size_x: 70.0,
        aperture_size_y: 51.0,
    };
    pub const VISTA_VISION: Aperture = Aperture {
        aperture_size_x: 37.72,
        aperture_size_y: 24.92,
    };
    pub const DYNA_VISION: Aperture = Aperture {
        aperture_size_x: 52.83,
        aperture_size_y: 37.59,
    };
    pub const IMAX: Aperture = Aperture {
        aperture_size_x: 70.41,
        aperture_size_y: 52.63,
    };
}
