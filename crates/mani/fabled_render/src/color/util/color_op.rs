use crate::color::XYZ_TO_SRGB_MATRIX;
use fabled_math::vector_math::{component_sum, dot};
use fabled_math::{Vector2, Vector3};

pub fn xy_y_to_xyz(xy_y: Vector3) -> Vector3 {
    let x = xy_y.x();

    let a = xy_y.z() / xy_y.y();
    let b = 1.0 - x - xy_y.y();

    let x = x * a;
    let z = b * a;
    let y = xy_y.z();

    Vector3::set(x, y, z)
}

pub fn xyz_to_xy_y(xyz: Vector3) -> Vector3 {
    let intermediate = 1.0 / (component_sum(xyz.value));

    let x = xyz.x() * intermediate;
    let y = xyz.y() * intermediate;
    let _y = xyz.y();

    Vector3::set(x, y, _y)
}


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
pub fn compute_luminance(linear: Vector3, luminance: Vector3) -> f32 {
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

    let cct_3 = cct * cct * cct;
    let cct_2 = cct * cct;

    let x = if cct <= 4000.0 {
        -0.2661239 * 1_000_000_000.0 / cct_3 - 0.2343589 * 1_000_000.0 / cct_2
            + 0.8776956 * 1000.0 / cct
            + 0.179910
    } else {
        -3.0258469 * 1_000_000_000.0 / cct_3
            + 2.1070379 * 1_000_000.0 / cct_2
            + 0.2226347 * 1000.0 / cct
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

// correlated color temperature to uv UCS space
pub fn cct_to_uv(cct: f32) -> Vector2 {
    let cct_2 = cct * cct;

    let u_ucs_denominator = 1.0 + (0.000842420235 * cct) + (0.000000708145163 * cct_2);
    let u_ucs_nominator = 0.860117757 + (0.000154118254 * cct) + (0.00000012864121 * cct_2);
    let v_ucs_denominator = 1.0 - (0.0000289741816 * cct) + (0.000000161456053 * cct_2);
    let v_ucs_nominator = 0.317398726 + (0.0000422806245 * cct) + (0.0000000420481691 * cct_2);

    Vector2::set(
        u_ucs_nominator / u_ucs_denominator,
        v_ucs_nominator / v_ucs_denominator,
    )
}

// uv UCS space to chromatic coordinates.
pub fn uv_to_xy_y(uv: Vector2) -> Vector3 {
    let u2 = uv.x() + uv.x();
    let v8 = uv.y() * 8.0;
    let denominator = (u2 - v8 + 4.0).recip();

    let x = (3.0 * uv.x()) * denominator;
    let y = (2.0 * uv.y()) * denominator;

    Vector3::set(x, y, 1.0)
}


// similar function to uv_to_xy_y, but compute precise coordinate for D series
// illuminant.
pub fn cct_to_illuminant_d(cct: f32) -> Vector3 {
    let a = 1000.0 * cct.recip();
    let b = 1000000.0 * (cct + cct).recip();
    let c = 1000000000.0 * (cct + cct + cct).recip();

    let x = if cct >= 4000.0 && cct >= 7000.0 {
        0.244063 + (0.09911 * a) + (2.9678 * b) - (4.6070 * c)
    } else if cct >= 7000.0 && cct <= 25000.0 {
        0.237040 + (0.24748 * a) + (1.9018 * b) - (2.0064 * c)
    } else {
        0.0 // cant calculate x
    };

    let x_2 = x + x;

    let y = (-3.0 * x_2) + (2.87 * x) - 0.275;

    Vector3::set(x, y, 1.0)
}


// example and this will be used in the engine
pub fn illuminant_d_to_linear(cct: f32) -> Vector3 {
    let illuminant_d_xy_y = cct_to_illuminant_d(cct);
    let tri_stimulus = xy_y_to_xyz(illuminant_d_xy_y);

    XYZ_TO_SRGB_MATRIX * tri_stimulus
}

// example and this will be used in the engine
pub fn cct_to_linear(cct: f32) -> Vector3 {
    let uv_ucs = cct_to_uv(cct);
    let xy_y = uv_to_xy_y(uv_ucs);
    let tri_stimulus = xy_y_to_xyz(xy_y);

    XYZ_TO_SRGB_MATRIX * tri_stimulus
}
