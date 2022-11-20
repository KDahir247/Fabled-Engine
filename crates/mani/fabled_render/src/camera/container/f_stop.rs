// Aperture F-stop usually ranges from 1 to 32 in standard increments: 1.4, 2,
// 2.8, 4, 5.6, 8, 11, 16, 22, and 32. The lower the number is on the F-stop the
// larger the opening, which mean more light will reach the sensor and the
// higher the number the less light will reach the sensor. This will affect
// exposure.

// The wider opening at lower f-stops narrows the range of focus so the image
// background is blurry when the foreground is in focus, and vice versa
// the higher the F-stop the more sharp and focus the background will be. While
// the lower the F-stop the more blurry the background will be. This will affect
// Depth of Field

use std::fmt::{Display, Formatter};

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq)]
pub struct FStop {
    pub f_stop: f32,
    pub step: i32,
}

impl Default for FStop {
    fn default() -> Self {
        FStop::F4_STOP
    }
}

impl FStop {
    pub fn new(f_number: f32, f_step: i32) -> Self {
        Self {
            f_stop: f_number,
            step: f_step,
        }
    }
}

impl FStop {
    // f/1 stop
    pub const F1_STOP: FStop = FStop {
        f_stop: 1.0,
        step: 0,
    };

    // f/1.4 stop
    pub const F1_4_STOP: FStop = FStop {
        f_stop: 1.4,
        step: 1,
    };

    // f/2 stop
    pub const F2_STOP: FStop = FStop {
        f_stop: 2.0,
        step: 2,
    };

    // f/2.8 stop
    pub const F2_8_STOP: FStop = FStop {
        f_stop: 2.8,
        step: 3,
    };

    // f/4 stop
    pub const F4_STOP: FStop = FStop {
        f_stop: 4.0,
        step: 4,
    };

    // f/5.6 stop
    pub const F5_6_STOP: FStop = FStop {
        f_stop: 5.6,
        step: 5,
    };

    // f/8 stop
    pub const F8_STOP: FStop = FStop {
        f_stop: 8.0,
        step: 6,
    };

    // f/11 stop
    pub const F11_STOP: FStop = FStop {
        f_stop: 11.0,
        step: 7,
    };

    // f/16 stop
    pub const F16_STOP: FStop = FStop {
        f_stop: 16.0,
        step: 8,
    };

    // f/22 stop
    pub const F22_STOP: FStop = FStop {
        f_stop: 22.0,
        step: 9,
    };

    // f/32 stop
    pub const F32_STOP: FStop = FStop {
        f_stop: 32.0,
        step: 10,
    };
}

impl Display for FStop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FStop(full stop : {}, step : {})",
            self.f_stop, self.step
        )
    }
}
