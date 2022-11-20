use fabled_component::Component;

use crate::camera::{inch_to_millimeter, meter_to_millimeter, MeasurementType};
use fabled_component::Untracked;

// Optional add component if camera is Perspective.

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq)]
pub struct Aperture {
    pub aperture_x_mm: f32,
    pub aperture_y_mm: f32,
}

impl Aperture {
    pub fn from_measurement(width: f32, height: f32, measurement: MeasurementType) -> Aperture {
        let (millimeter_aperture_x, millimeter_aperture_y) = match measurement {
            MeasurementType::Inch => (inch_to_millimeter(width), inch_to_millimeter(height)),
            MeasurementType::Meter => (meter_to_millimeter(width), meter_to_millimeter(height)),
            MeasurementType::Millimeter => (width, height),
        };

        Aperture::new(millimeter_aperture_x, millimeter_aperture_y)
    }

    pub const fn new(mm_width: f32, mm_height: f32) -> Aperture {
        Aperture {
            aperture_x_mm: mm_width,
            aperture_y_mm: mm_height,
        }
    }

    pub const R8MM: Aperture = Aperture {
        aperture_x_mm: 4.8,
        aperture_y_mm: 3.5,
    };
    pub const R8MM_SUPER: Aperture = Aperture {
        aperture_x_mm: 5.79,
        aperture_y_mm: 4.01,
    };
    pub const R16MM_THEATRICAL: Aperture = Aperture {
        aperture_x_mm: 10.26,
        aperture_y_mm: 7.49,
    };
    pub const SUPER_16MM: Aperture = Aperture {
        aperture_x_mm: 12.52,
        aperture_y_mm: 7.41,
    };
    pub const ULTRA_16MM: Aperture = Aperture {
        aperture_x_mm: 11.66,
        aperture_y_mm: 7.49,
    };
    pub const R35MM_ACADEMY: Aperture = Aperture {
        aperture_x_mm: 21.0,
        aperture_y_mm: 15.2,
    };
    pub const R35MM_2PERF: Aperture = Aperture {
        aperture_x_mm: 21.95,
        aperture_y_mm: 9.35,
    };
    pub const R35MM_TECHNISCOPE: Aperture = Aperture {
        aperture_x_mm: 22.0,
        aperture_y_mm: 9.47,
    };
    pub const R35MM_185_PROJECTION: Aperture = Aperture {
        aperture_x_mm: 22.0,
        aperture_y_mm: 16.0,
    };
    pub const R35MM_ANAMORPHIC: Aperture = Aperture {
        aperture_x_mm: 21.95,
        aperture_y_mm: 18.6,
    };
    pub const SUPER_35MM: Aperture = Aperture {
        aperture_x_mm: 24.89,
        aperture_y_mm: 18.66,
    };
    pub const R35_ALEXA: Aperture = Aperture {
        aperture_x_mm: 54.12,
        aperture_y_mm: 23.59,
    };
    pub const R70MM: Aperture = Aperture {
        aperture_x_mm: 70.0,
        aperture_y_mm: 51.0,
    };
    pub const VISTA_VISION: Aperture = Aperture {
        aperture_x_mm: 37.72,
        aperture_y_mm: 24.92,
    };
    pub const DYNA_VISION: Aperture = Aperture {
        aperture_x_mm: 52.83,
        aperture_y_mm: 37.59,
    };
    pub const IMAX: Aperture = Aperture {
        aperture_x_mm: 70.41,
        aperture_y_mm: 52.63,
    };
}

impl Component for Aperture {
    type Tracking = Untracked;
}
