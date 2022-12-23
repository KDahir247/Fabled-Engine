use fabled_math::vector_math::dot;
use fabled_math::{Vector2, Vector3};


pub fn chromatic_coord_to_tri_stimulus_white(chromaticity_coord: Vector2) -> Vector3 {
    let denometer = chromaticity_coord.y().recip();

    Vector3::set(
        chromaticity_coord.x() * denometer,
        1.0,
        (1.0 - chromaticity_coord.x() - chromaticity_coord.y()) * denometer,
    )
}

pub fn tri_stimulus_white_to_chromatic_coord(chromatic_coord: Vector3) -> Vector2 {
    let denometer = (chromatic_coord.x() + chromatic_coord.y() + chromatic_coord.z()).recip();
    Vector2::set(
        chromatic_coord.x() * denometer,
        chromatic_coord.y() * denometer,
    )
}

// ACEScct and ACEScc has relatively the same relative luminance as srgb refer
// to https://docs.acescentral.com/specifications/acescct/#appendix-a-application-of-asc-cdl-parameters-to-acescct-image-data
// https://docs.acescentral.com/specifications/acescc/#appendix-b-application-of-asc-cdl-parameters-to-acescc-image-data
// Use util\mod.rs pre-determined color luminance value (DCI-P3, SRGB, AP1,)
pub fn srgb_compute_luminance(linear: Vector3, luminance: Vector3) -> f32 {
    dot(linear.value, luminance.value)
}


pub fn compute_perceived_lightness(luminance: f32) -> f32 {
    let luminance = luminance.clamp(0.0, 1.0);

    let mut lightness = luminance * 903.3;

    if luminance > 0.008856 {
        lightness = luminance.powf(1.0 / 3.0) * 116.0 - 16.0;
    }

    lightness
}


pub fn cct_to_chromatic_coord(cct: f32) -> Vector2 {
    let cct = cct.clamp(1667.0, 25000.0);

    let cct_3 = cct.powf(3.0f32);
    let cct_2 = cct.powf(2.0f32);

    let x = if cct <= 4000.0 {
        -0.2661239 * 10.0f32.powf(9.0) / cct_3 - 0.2343589 * 10.0f32.powf(6.0) / cct_2
            + 0.8776956 * 10.0f32.powf(3.0) / cct
            + 0.179910
    } else {
        -3.0258469 * 10.0f32.powf(9.0) / cct_3
            + 2.1070379 * 10.0f32.powf(6.0) / cct_2
            + 0.2226347 * 10.0f32.powf(3.0) / cct
            + 0.24039
    };

    let x_3 = x * x * x;
    let x_2 = x * x;

    let y = if cct <= 2222.0 {
        -1.1063814 * x_3 - 1.34811020 * x_2 + 2.18555832 * x - 0.20219683
    } else if cct > 2222.0 && cct <= 4000.0 {
        -0.9549476 * x_3 - 1.37418593 * x_2 + 2.09137015 * x - 0.16748867
    } else {
        3.0817580 * x_3 - 5.8733867 * x_2 + 3.75112997 * x - 0.37001483
    };

    Vector2::set(x, y)
}
