// N is the relative aperture (f-number)
// t is the exposure time ("shutter speed") in seconds
// L is the average scene luminance
// S is the ISO arithmetic speed
// K is the reflected-light meter calibration constant

// EV = log2 (LS / K)
// (LS / K) is equivalent to N^2 / t, thus EV = log2 (N^2 / t)

use crate::light::{FStop, FullStop, Shutter};

pub fn calculate_exposure_value(f_stop: FStop) -> f32 {
    let shutter = Shutter::compute_shutter_speed(f_stop);

    let mut target_stop = FullStop::f4_stop();

    match f_stop {
        FStop::FullStop(full_stop) => {
            target_stop = full_stop;
        }
        _ => unimplemented!(),
    };

    let f_number = target_stop.get_f_number();

    f32::log2(f_number * f_number / shutter.get_shutter_speed())
}
