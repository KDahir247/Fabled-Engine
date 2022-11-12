// T is the Transmission (Percentage of light that is not absorbed by the
// optics in the lens)
// V(theta) is the vignetting factor (The characteristic of light falloff from
// the center of the image to the edge)
// Theta is the angle relative to the axis of the lens.

// vignette = 4/PI * V(theta) * cos(theta)^4
pub fn calculate_len_vignette_attenuation(
    transmission_percentage: f32,
    vignette_factor: f32,
    angle_axis_len_rad: f32,
) -> f32 {
    let transmission_decimal_form = transmission_percentage * 0.01;

    let angle_axis_len_cos = angle_axis_len_rad.cos();

    1.273239545f32
        * (vignette_factor * transmission_decimal_form)
        * (angle_axis_len_cos * angle_axis_len_cos)
        * (angle_axis_len_cos * angle_axis_len_cos)
}


// Decompose for transmission, vignette_factor, and angle_axis_len will go here.
