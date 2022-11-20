use crate::camera::{
    compute_ev_100_with_camera_properties, FStop, ISOSpeed, ISOSpeedUnit, Shutter,
    ISO_ARITHMETIC_STANDARD, SHUTTER_SPEED_STANDARD,
};


// N is the relative aperture (f-number)
// t is the exposure time ("shutter speed") in seconds
// L is the average scene luminance
// S is the ISO arithmetic speed

// S = (N^2 * 100) / (t * 2^ev100)
pub fn compute_iso_speed(aperture: FStop, shutter: Shutter, ev100: f32) -> ISOSpeed {
    let f_stop_sqr = aperture.f_stop * aperture.f_stop;

    let two_pow_ev = 2.0f32.powf(ev100);

    let iso_speed = (f_stop_sqr * 100.0) / (shutter.speed_second * two_pow_ev);

    ISOSpeed::new(iso_speed, ISOSpeedUnit::Arithmetic)
}

// focal length is measured in mm
pub fn apply_aperture_priority(
    aperture: FStop,
    focal_length: f32,
    target_ev: f32,
) -> (ISOSpeed, Shutter) {
    // if you are shooting with a 200mm lens then you want to keep your shutter
    // speed at 1/200 sec or above to avoid any blur occurring from camera shake
    let shutter = Shutter::new(1.0 / focal_length * 1000.0);

    let iso = compute_iso_speed(aperture, shutter, target_ev);

    let ev_diff = target_ev - compute_ev_100_with_camera_properties(aperture, shutter, iso);

    let shutter_speed_exact = (shutter.speed_second * 2.0f32.powf(-ev_diff)).clamp(
        SHUTTER_SPEED_STANDARD[0],
        SHUTTER_SPEED_STANDARD[SHUTTER_SPEED_STANDARD.len() - 1],
    );

    let shutter = Shutter::new(shutter_speed_exact);
    (iso, shutter)
}

pub fn apply_shutter_priority(shutter: Shutter, target_ev: f32) -> (FStop, ISOSpeed) {
    let aperture = FStop::F4_STOP;

    let iso_speed = compute_iso_speed(aperture, shutter, target_ev)
        .arithmetic_speed
        .clamp(
            ISO_ARITHMETIC_STANDARD[0],
            ISO_ARITHMETIC_STANDARD[ISO_ARITHMETIC_STANDARD.len() - 1],
        );

    let iso = ISOSpeed::new(iso_speed, ISOSpeedUnit::Arithmetic);

    let ev_diff = target_ev - compute_ev_100_with_camera_properties(aperture, shutter, iso);

    let final_aperture =
        (aperture.f_stop * std::f32::consts::SQRT_2.powf(ev_diff)).clamp(1.0, 32.0);

    // We can't really figure the step for the F-Stop out.
    (FStop::new(final_aperture, 0), iso)
}

// calculate aperture and shutter
pub fn compute_camera_properties(focal_length: f32, target_ev: f32) -> (FStop, Shutter, ISOSpeed) {
    let aperture = FStop::F4_STOP;

    let shutter = Shutter::new(1.0f32 / (focal_length * 1000.0));

    let iso_speed = compute_iso_speed(aperture, shutter, target_ev)
        .arithmetic_speed
        .clamp(
            ISO_ARITHMETIC_STANDARD[0],
            ISO_ARITHMETIC_STANDARD[ISO_ARITHMETIC_STANDARD.len() - 1],
        );

    let res_iso = ISOSpeed::new(iso_speed, ISOSpeedUnit::Arithmetic);

    let ev_diff = target_ev - compute_ev_100_with_camera_properties(aperture, shutter, res_iso);

    let f_stop = (aperture.f_stop * std::f32::consts::SQRT_2.powf(ev_diff * 0.5)).clamp(1.0, 32.0);

    let res_aperture = FStop::new(f_stop, 0);

    let ev_diff = target_ev - compute_ev_100_with_camera_properties(res_aperture, shutter, res_iso);

    let res_shutter_speed = shutter.speed_second
        * 2.0f32.powf(-ev_diff).clamp(
            SHUTTER_SPEED_STANDARD[0],
            SHUTTER_SPEED_STANDARD[SHUTTER_SPEED_STANDARD.len() - 1],
        );

    let res_shutter = Shutter::new(res_shutter_speed);

    (res_aperture, res_shutter, res_iso)
}
