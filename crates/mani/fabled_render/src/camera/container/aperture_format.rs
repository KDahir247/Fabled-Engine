// Camera aperture (sensor size in unity)
// size of camera sensor in millimeters

#[non_exhaustive]
#[repr(align(8))]
#[derive(Debug, Copy, Clone, Default)]
pub struct Aperture {
    pub aperture_size_x: f32,
    pub aperture_size_y: f32,
    pub aperture_aspect_ratio: f32,
}
impl Aperture {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            aperture_size_x: width,
            aperture_size_y: height,
            aperture_aspect_ratio: width / height,
        }
    }

    pub const R8MM: Aperture = Aperture {
        aperture_size_x: 4.8,
        aperture_size_y: 3.5,
        aperture_aspect_ratio: 4.8 / 3.5,
    };
    pub const R8MM_SUPER: Aperture = Aperture {
        aperture_size_x: 5.79,
        aperture_size_y: 4.01,
        aperture_aspect_ratio: 5.79 / 4.01,
    };
    pub const R16MM_THEATRICAL: Aperture = Aperture {
        aperture_size_x: 10.26,
        aperture_size_y: 7.49,
        aperture_aspect_ratio: 10.26 / 7.49,
    };
    pub const SUPER_16MM: Aperture = Aperture {
        aperture_size_x: 12.52,
        aperture_size_y: 7.41,
        aperture_aspect_ratio: 12.52,
    };
    pub const ULTRA_16MM: Aperture = Aperture {
        aperture_size_x: 11.66,
        aperture_size_y: 7.49,
        aperture_aspect_ratio: 11.66 / 7.49,
    };
    pub const R35MM_ACADEMY: Aperture = Aperture {
        aperture_size_x: 21.0,
        aperture_size_y: 15.2,
        aperture_aspect_ratio: 21.0 / 15.2,
    };
    pub const R35MM_2PERF: Aperture = Aperture {
        aperture_size_x: 21.95,
        aperture_size_y: 9.35,
        aperture_aspect_ratio: 21.95 / 9.35,
    };
    pub const R35MM_TECHNISCOPE: Aperture = Aperture {
        aperture_size_x: 22.0,
        aperture_size_y: 9.47,
        aperture_aspect_ratio: 22.0 / 9.47,
    };
    pub const R35MM_185_PROJECTION: Aperture = Aperture {
        aperture_size_x: 22.0,
        aperture_size_y: 16.0,
        aperture_aspect_ratio: 22.0 / 16.0,
    };
    pub const R35MM_ANAMORPHIC: Aperture = Aperture {
        aperture_size_x: 21.95,
        aperture_size_y: 18.6,
        aperture_aspect_ratio: 21.95 / 18.6,
    };
    pub const SUPER_35MM: Aperture = Aperture {
        aperture_size_x: 24.89,
        aperture_size_y: 18.66,
        aperture_aspect_ratio: 24.89 / 18.66,
    };
    pub const R35_ALEXA: Aperture = Aperture {
        aperture_size_x: 54.12,
        aperture_size_y: 25.59,
        aperture_aspect_ratio: 54.12 / 25.59,
    };
    pub const R70MM: Aperture = Aperture {
        aperture_size_x: 70.0,
        aperture_size_y: 51.0,
        aperture_aspect_ratio: 70.0 / 51.0,
    };
    pub const VISTA_VISION: Aperture = Aperture {
        aperture_size_x: 37.72,
        aperture_size_y: 24.92,
        aperture_aspect_ratio: 37.72 / 24.92,
    };
    pub const DYNA_VISION: Aperture = Aperture {
        aperture_size_x: 52.83,
        aperture_size_y: 37.59,
        aperture_aspect_ratio: 52.83 / 37.59,
    };
    pub const IMAX: Aperture = Aperture {
        aperture_size_x: 70.41,
        aperture_size_y: 52.63,
        aperture_aspect_ratio: 70.41 / 52.63,
    };
}
