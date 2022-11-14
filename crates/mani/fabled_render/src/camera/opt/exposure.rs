use crate::camera::{FStop, ISOSpeed, ISOSpeedUnit, Shutter, LENS_VIGNETTING_ATTENUATION};

// N is the relative aperture (f-number)
// t is the exposure time ("shutter speed") in seconds
// L is the average scene luminance
// S is the ISO arithmetic speed
// K is the reflected-light meter calibration constant ISO 2720:1974 recommended
// value 10.6 and 13.4
// 12.5 (Canon, Nikon, Sekonic) and 14.0 (Minolta, Kenko, Pentax)

// Only take into account Aperture and Shutter speed not sensitivity.
// EV = log2 (LS / K)
// (LS / K) is equivalent to N^2 / t, thus EV = log2 (N^2 / t)
// Ev100
pub fn compute_exposure_value(f_stop: FStop, shutter: Option<Shutter>) -> f32 {
    let shutter = shutter.unwrap_or(Shutter::compute_shutter_speed(f_stop));

    let f_number = f_stop.f_stop;

    f32::log2(f_number * f_number / shutter.speed)
}

pub fn compute_ev_100_with_camera_properties(
    aperture: FStop,
    shutter: Shutter,
    iso_speed: ISOSpeed,
) -> f32 {
    let f_stop_sqr = aperture.f_stop * aperture.f_stop;

    ((f_stop_sqr * 100.0) / (shutter.speed * iso_speed.arithmetic_speed)).log2()
}

// We want to compute exposure value with Sensitivity.

// EV100 = log2(N^2 / t) - log2(S / 100.0)
pub fn compute_ev_100(f_stop: FStop, iso_speed: ISOSpeed) -> f32 {
    let ev_s = compute_exposure_value(f_stop, None);
    ev_s - f32::log2(iso_speed.arithmetic_speed / 100.0)
}

// Ls is Scene luminance
// K is light meter calibration constant (usually 12.5)
// EV100 is Exposure value confined 100

// EV100 = Log2(LS * 100.0 / K)
// EV100 = Log2(LS * 100.0 / 12.5)
// where EV and ISO is confined to EV100 and ISO100
fn compute_ev_100_from_scene_luminance(scene_luminance: f32) -> f32 {
    // 8 is from 100/12.5
    (scene_luminance * 8.0).log2()
}

// EV100 is Exposure value confined to 100
// Ls is Scene luminance

// Ls = 2.0^(ev100 - 3.0)
fn compute_scene_luminance_from_ev_100(ev_100: f32) -> f32 {
    2.0f32.powf(ev_100 - 3.0)
}

// EV100' = EV100 - EC
// Allow user to apply compensation in order to over-expose (brighten) or
// under-expose (darken)
pub fn compute_ev100_with_compensation(ev100: f32, exposure_compensation: f32) -> f32 {
    ev100 - exposure_compensation
}

// todo this is what we pass to the shader. color.rgb *= exposure.
// H is the photometric exposure, expressed in lux seconds
// q is the lens and vignetting attenuation factor (usually 0.65^11)
// t is the exposure time ("shutter speed") in seconds
// L is the luminance of the scene
// N is the relative aperture (f-number)
// S is the ISO arithmetic speed

// Convert EV100 to Exposure for shader
// The SI unit for luminance is candela per square metre (cd/m2)
// L' =L*1/Lmax
// Lmax = (N^2/t) * (78 / q * S)
// which is equivalent to Lmax =2^EV100 * (78 / q * S)
pub fn compute_exposure_normalization_factor(
    ev100: f32,
    len_attenuation: Option<f32>,
    iso_speed: ISOSpeed,
) -> f32 {
    let len_attenuation = len_attenuation.unwrap_or(LENS_VIGNETTING_ATTENUATION);

    // todo since we specify ev100 in parameter we should conform iso to iso 100
    //  too so (78.9 / (len_atten * iso_speed.arithmetic_speed should be 1.2)).

    1.0 / (2.0f32.powf(ev100) * (78.0 / (len_attenuation * iso_speed.arithmetic_speed)))
}
