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

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum FStop {
    FullStop(FullStop),
    // todo
    // not sure if this should be implemented for precise f stop
    // HalfStop,
    // todo
    // not sure if this should be implemented for precise f stop
    // ThirdStop,
}

#[derive(Copy, Clone, Debug)]
pub struct FullStop {
    f_stop: f32,
    step: i32,
}

impl Default for FullStop {
    fn default() -> Self {
        FullStop::F4_STOP
    }
}

impl FullStop {
    // todo make const.
    // f/1 stop
    pub const F1_STOP: FullStop = FullStop {
        f_stop: 1.0,
        step: 0,
    };

    // f/1.4 stop
    pub const F1_4_STOP: FullStop = FullStop {
        f_stop: 1.4,
        step: 1,
    };

    // f/2 stop
    pub const F2_STOP: FullStop = FullStop {
        f_stop: 2.0,
        step: 2,
    };

    // f/2.8 stop
    pub const F2_8_STOP: FullStop = FullStop {
        f_stop: 2.8,
        step: 3,
    };

    // f/4 stop
    pub const F4_STOP: FullStop = FullStop {
        f_stop: 4.0,
        step: 4,
    };

    // f/5.6 stop
    pub const F5_6_STOP: FullStop = FullStop {
        f_stop: 5.6,
        step: 5,
    };

    // f/8 stop
    pub const F8_STOP: FullStop = FullStop {
        f_stop: 8.0,
        step: 6,
    };

    // f/11 stop
    pub const F11_STOP: FullStop = FullStop {
        f_stop: 11.0,
        step: 7,
    };

    // f/16 stop
    pub const F16_STOP: FullStop = FullStop {
        f_stop: 16.0,
        step: 8,
    };

    // f/22 stop
    pub const F22_STOP: FullStop = FullStop {
        f_stop: 22.0,
        step: 9,
    };

    // f/32 stop
    pub const F32_STOP: FullStop = FullStop {
        f_stop: 32.0,
        step: 10,
    };
}


impl FullStop {
    pub const fn get_f_number(&self) -> f32 {
        self.f_stop
    }

    pub const fn get_step(&self) -> i32 {
        self.step
    }
}

// 1/2 Stop values:
// f/1, f/1.2, f/1.4, f/1.7, f/2, f/2.4, f/2.8, f/3.3, f/4,
// f/4.8, f/5.6, f/6.7, f/8, f/9.5, f/11, f/13, f/16, f/19, f/22, f/27, f/32

// 1/3 Stop values:
// f/1, f/1.1, f/1.2, f/1.4, f/1.6, f/1.8, f/2, f/2.2, f/2.5,
// f/2.8, f/3.2, f/3.5, f/4.0, f/4.5, f/5, f/5.6, f/6.3, f/7.1, f/8, f/9, f/10,
// f/11, f/13, f/14, f/16, f/18, f/20, f/22, f/25, f/29, f/32, f/36
