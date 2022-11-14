// Camera aperture (sensor size in unity)
// size of camera sensor in millimeters

use fabled_math::Vector2;

#[non_exhaustive]
#[derive(Copy, Clone, Default)]
pub struct Aperture {
    pub aperture_size: Vector2,
}

impl Aperture {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            aperture_size: Vector2::set(width, height),
        }
    }

    pub const R8MM: Aperture = Aperture {
        aperture_size: Vector2::set(4.8, 3.5),
    };

    pub const R8MM_SUPER: Aperture = Aperture {
        aperture_size: Vector2::set(5.79, 4.01),
    };
    pub const R16MM_THEATRICAL: Aperture = Aperture {
        aperture_size: Vector2::set(10.26, 7.49),
    };
    pub const SUPER_16MM: Aperture = Aperture {
        aperture_size: Vector2::set(12.52, 7.41),
    };
    pub const ULTRA_16MM: Aperture = Aperture {
        aperture_size: Vector2::set(11.66, 7.49),
    };
    pub const R35MM_ACADEMY: Aperture = Aperture {
        aperture_size: Vector2::set(21.0, 15.2),
    };
    pub const R35MM_2PERF: Aperture = Aperture {
        aperture_size: Vector2::set(21.95, 9.35),
    };
    pub const R35MM_TECHNISCOPE: Aperture = Aperture {
        aperture_size: Vector2::set(22.0, 9.47),
    };
    pub const R35MM_185_PROJECTION: Aperture = Aperture {
        aperture_size: Vector2::set(22.0, 16.0),
    };
    pub const R35MM_ANAMORPHIC: Aperture = Aperture {
        aperture_size: Vector2::set(21.95, 18.6),
    };
    pub const SUPER_35MM: Aperture = Aperture {
        aperture_size: Vector2::set(24.89, 18.66),
    };

    pub const R35_ALEXA: Aperture = Aperture {
        aperture_size: Vector2::set(54.12, 23.59),
    };
    pub const R70MM: Aperture = Aperture {
        aperture_size: Vector2::set(70.0, 51.0),
    };
    pub const VISTA_VISION: Aperture = Aperture {
        aperture_size: Vector2::set(37.72, 24.92),
    };
    pub const DYNA_VISION: Aperture = Aperture {
        aperture_size: Vector2::set(52.83, 37.59),
    };
    pub const IMAX: Aperture = Aperture {
        aperture_size: Vector2::set(70.41, 52.63),
    };
}
