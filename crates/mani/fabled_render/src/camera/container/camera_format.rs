use crate::camera::AspectRatio;
use fabled_math::Vector2;
use std::fmt::{Display, Formatter};

#[non_exhaustive]
pub struct CameraFormat {
    pub aspect_ratio: AspectRatio,
    pub resolution_pixel: Vector2,
}


impl CameraFormat {
    pub const fn custom(aspect_ratio: AspectRatio, resolution_pixel: Vector2) -> CameraFormat {
        CameraFormat {
            aspect_ratio,
            resolution_pixel,
        }
    }
}


impl CameraFormat {
    pub const R128X128: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 1.0,
            vertical: 1.0,
        },
        Vector2::set(128.0, 128.0),
    );

    pub const D1NTSC: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 4.0,
            vertical: 3.0,
        },
        Vector2::set(720.0, 486.0),
    );

    pub const NTSC: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 4.0,
            vertical: 3.0,
        },
        Vector2::set(440.0, 486.0),
    );

    pub const PAL: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 4.0,
            vertical: 3.0,
        },
        Vector2::set(520.0, 576.0),
    );

    pub const D1PAL: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 4.0,
            vertical: 3.0,
        },
        Vector2::set(720.0, 576.0),
    );

    pub const R640X480: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 4.0,
            vertical: 3.0,
        },
        Vector2::set(640.0, 480.0),
    );

    pub const R320X200: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 4.0,
            vertical: 3.0,
        },
        Vector2::set(320.0, 200.0),
    );


    // Standard

    pub const HD43: CameraFormat =
        CameraFormat::custom(AspectRatio::new(4.0, 3.0), Vector2::set(1440.0, 1080.0));

    pub const P720: CameraFormat =
        CameraFormat::custom(AspectRatio::new(16.0, 9.0), Vector2::set(1280.0, 720.0));

    pub const HD: CameraFormat =
        CameraFormat::custom(AspectRatio::new(16.0, 9.0), Vector2::set(1366.0, 768.0));

    pub const HD_PLUS: CameraFormat =
        CameraFormat::custom(AspectRatio::new(16.0, 9.0), Vector2::set(1600.0, 900.0));

    pub const FULL_HD: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 16.0,
            vertical: 9.0,
        },
        Vector2::set(1920.0, 1080.0),
    );

    pub const QUAD_HD: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 16.0,
            vertical: 9.0,
        },
        Vector2::set(2560.0, 1440.0),
    );

    pub const QUAD_HD_PLUS: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 16.0,
            vertical: 9.0,
        },
        Vector2::set(3200.0, 1800.0),
    );

    pub const ULTRA_HD: CameraFormat = CameraFormat::custom(
        AspectRatio {
            horizontal: 16.0,
            vertical: 9.0,
        },
        Vector2::set(3840.0, 2160.0),
    );
}

impl Display for CameraFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScreenFormat(\n\t{}\n\t{}\n)",
            self.aspect_ratio, self.resolution_pixel
        )
    }
}
