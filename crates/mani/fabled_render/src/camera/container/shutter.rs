use crate::camera::FStop;
// longer shutter speed result in blurrier image.
// shorter shutter speed will capture fasting-moving action and eliminate motion
// blur

// shutter speed also affect exposure. The faster the shutter speed the less
// exposure on sensor. While having a slow shutter speed will increase the
// exposure on sensor depending on how slow the shutter speed is.


#[derive(Copy, Clone, Debug)]
pub struct Shutter {
    pub speed: f32,
}

impl Shutter {
    pub fn compute_shutter_speed(f_stop: FStop) -> Self {
        let mut shutter = 4000.0;

        for _ in 0..f_stop.step {
            shutter /= 2.0;
        }
        Self {
            speed: 1.0 / shutter,
        }
    }

    pub fn new(shutter_speed: f32) -> Self {
        Self {
            speed: shutter_speed,
        }
    }
}


#[cfg(test)]
mod shutter_test {
    use crate::camera::{FStop, Shutter};

    #[test]
    fn retrieve_shutter() {
        let mut shutter = Shutter::compute_shutter_speed(FStop::F1_4_STOP);

        let initial_shutter = 4000.0;
        let f1_4_shutter = 1.0 / (initial_shutter / 2.0);
        let f2_shutter = 1.0 / (initial_shutter / 2.0 / 2.0);
        let f2_8_shutter = 1.0 / (initial_shutter / 2.0 / 2.0 / 2.0);
        let f4_shutter = 1.0 / (initial_shutter / 2.0 / 2.0 / 2.0 / 2.0);
        let f5_6_shutter = 1.0 / (initial_shutter / 2.0 / 2.0 / 2.0 / 2.0 / 2.0);
        let f8_shutter = 1.0 / (initial_shutter / 2.0 / 2.0 / 2.0 / 2.0 / 2.0 / 2.0);


        assert!(shutter.speed.eq(&f1_4_shutter));

        shutter = Shutter::compute_shutter_speed(FStop::F2_STOP);

        assert!(shutter.speed.eq(&f2_shutter));

        shutter = Shutter::compute_shutter_speed(FStop::F2_8_STOP);

        assert!(shutter.speed.eq(&f2_8_shutter));

        shutter = Shutter::compute_shutter_speed(FStop::F4_STOP);

        assert!(shutter.speed.eq(&f4_shutter));

        shutter = Shutter::compute_shutter_speed(FStop::F5_6_STOP);

        assert!(shutter.speed.eq(&f5_6_shutter));

        shutter = Shutter::compute_shutter_speed(FStop::F8_STOP);

        assert!(shutter.speed.eq(&f8_shutter));
    }
}
