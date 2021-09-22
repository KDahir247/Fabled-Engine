#[derive(Copy, Clone, Debug)]
pub enum FStop {
    FullStop(FullStop),
    /* not sure if this should be implemented for precise f stop
     * HalfStop,
     * not sure if this should be implemented for precise f stop
     * ThirdStop, */
}

#[derive(Copy, Clone, Debug)]
pub struct FullStop {
    f_stop: f32,
    step: i32,
}

impl Default for FullStop {
    fn default() -> Self {
        Self::f4_stop()
    }
}

impl FullStop {
    // f/1 stop
    pub fn f1_stop() -> FullStop {
        Self {
            f_stop: 1.0,
            step: 0,
        }
    }

    // f/1.4 stop
    pub fn f1_4_stop() -> FullStop {
        Self {
            f_stop: 1.4,
            step: 1,
        }
    }

    // f/2 stop
    pub fn f2_stop() -> FullStop {
        Self {
            f_stop: 2.0,
            step: 2,
        }
    }

    // f/2.8 stop
    pub fn f2_8_stop() -> FullStop {
        Self {
            f_stop: 2.8,
            step: 3,
        }
    }

    // f/4 stop
    pub fn f4_stop() -> FullStop {
        Self {
            f_stop: 4.0,
            step: 4,
        }
    }

    // f/5.6 stop
    pub fn f5_6_stop() -> FullStop {
        Self {
            f_stop: 5.6,
            step: 5,
        }
    }


    // f/8 stop
    pub fn f8_stop() -> FullStop {
        Self {
            f_stop: 8.0,
            step: 6,
        }
    }
}


impl FullStop {
    pub fn get_f_number(&self) -> f32 {
        self.f_stop
    }

    pub fn get_step(&self) -> i32 {
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
