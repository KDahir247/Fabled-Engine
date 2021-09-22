use crate::camera::{FStop, ISOSpeed, Shutter, LENS_VIGNETTING_ATTENUATION};

// N is the relative aperture (f-number)
// t is the exposure time ("shutter speed") in seconds
// L is the average scene luminance
// S is the ISO arithmetic speed
// K is the reflected-light meter calibration constant ISO 2720:1974 recommended
// value 10.6 and 13.4
// 12.5 (Canon, Nikon, Sekonic) and 14.0 (Minolta, Kenko, Pentax)


// EV = log2 (LS / K)
// (LS / K) is equivalent to N^2 / t, thus EV = log2 (N^2 / t)
pub fn calculate_exposure_value(f_stop: FStop) -> f32 {
    let shutter = Shutter::compute_shutter_speed(f_stop);

    let FStop::FullStop(target_stop) = f_stop;

    let f_number = target_stop.get_f_number();

    f32::log2(f_number * f_number / shutter.speed)
}

// EV100 = log2(N^2 / t) - log2(S / 100.0)
pub fn calculate_ev_100(f_stop: FStop, iso_speed: ISOSpeed) -> f32 {
    let ev_s = calculate_exposure_value(f_stop);
    ev_s - f32::log2(iso_speed.arithmetic_speed / 100.0)
}


// EV100' = EV100 - EC
pub fn ev100_compensation(ev100: f32, exposure_compensation: f32) -> f32 {
    ev100 - exposure_compensation
}

// H is the photometric exposure, expressed in lux seconds
// q is the lens and vignetting attenuation factor (usually 0.65^11)
// t is the exposure time ("shutter speed") in seconds
// L is the luminance of the scene
// N is the relative aperture (f-number)
// S is the ISO arithmetic speed

// The SI unit for luminance is candela per square metre (cd/m2)
// L' =L*1/Lmax
// Lmax = (N^2/t) * (78 / q * S)
// which is equivalent to Lmax =2^EV100 * (78 / q * S)
pub fn calculate_exposure_normalization_factor(
    ev100: f32,
    len_attenuation: Option<f32>,
    iso_speed: ISOSpeed,
) -> f32 {
    let len_attenuation = len_attenuation.unwrap_or(LENS_VIGNETTING_ATTENUATION);
    1.0 / (2.0f32.powf(ev100) * (78.0 / len_attenuation * iso_speed.arithmetic_speed))
}
