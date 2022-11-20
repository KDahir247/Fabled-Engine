use crate::camera::{FStop, ISOSpeed, Shutter, LENS_VIGNETTING_ATTENUATION};

pub fn compute_saturation_based_exposure(
    aperture: FStop,
    shutter_speed: Option<Shutter>,
    iso: ISOSpeed,
    vignetting_attenuation: Option<f32>,
) -> f32 {
    let shutter = shutter_speed.unwrap_or(Shutter::compute_shutter_speed(aperture));

    let vignetting_rcp = vignetting_attenuation
        .unwrap_or(LENS_VIGNETTING_ATTENUATION)
        .recip();

    let f_stop_sqr = aperture.f_stop * aperture.f_stop;
    let denominator_rcp = (shutter.speed_second * iso.arithmetic_speed).recip();

    let sat_speed_rate = (78.0 * vignetting_rcp) * (f_stop_sqr * denominator_rcp);

    1.0 / sat_speed_rate
}

// todo only hold 4 parameter.
pub fn compute_standard_based_exposure(
    aperture: FStop,
    shutter_speed: Option<Shutter>,
    iso: ISOSpeed,
    vignetting_attenuation: Option<f32>,
    mid_grey: Option<f32>,
) -> f32 {
    let shutter = shutter_speed.unwrap_or(Shutter::compute_shutter_speed(aperture));

    let vignetting_rcp = vignetting_attenuation
        .unwrap_or(LENS_VIGNETTING_ATTENUATION)
        .recip();

    let mid_grey = mid_grey.unwrap_or(0.18);

    let f_number_sqr = aperture.f_stop * aperture.f_stop;

    let denominator_rcp = (shutter.speed_second * iso.arithmetic_speed).recip();

    let stan_speed_rate = (10.0 * vignetting_rcp) * (f_number_sqr * denominator_rcp);

    mid_grey / stan_speed_rate
}
