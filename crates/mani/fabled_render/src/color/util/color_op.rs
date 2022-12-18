use crate::camera::{compute_exposure_value, FStop, Shutter};
use fabled_math::vector_math::{
    clamp, component_max, component_min, dot, exp2, gain, le, lerp, log2, lt, pow, rcp, select,
    sign, signum,
};
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

// Find luminance (Y) from linear srgb
// To find the luminance of a color space you need the COLORSPACE -> XYZ Matrix
// and get the Y ROW or Y COL if your matrix is a row or column major.
pub fn srgb_compute_luminance(linear: Vector3) -> f32 {
    0.21263682 * linear.x() + 0.71518298 * linear.y() + 0.0721802 * linear.z()
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

pub fn contrast(linear_srgb: Vector3, contrast: f32) -> Vector3 {
    Vector3 {
        value: gain(linear_srgb.value, Vector3::broadcast(1.0 - contrast).value),
    }
}


pub fn desaturate(linear_srgb: Vector3, factor: Vector3) -> Vector3 {
    let max_luminance = srgb_compute_luminance(linear_srgb);

    let grey = Vector3::broadcast(max_luminance);

    Vector3 {
        value: lerp(linear_srgb.value, grey.value, factor.value),
    }
}

pub fn saturate(linear_srgb: Vector3, factor: Vector3) -> Vector3 {
    let max_luminance = srgb_compute_luminance(linear_srgb);

    let gray = Vector3::broadcast(max_luminance);

    let diff = linear_srgb - gray;
    let saturate = gray + diff;

    Vector3 {
        value: clamp(
            (saturate * factor).value,
            Vector3::ZERO.value,
            Vector3::ONE.value,
        ),
    }
}

// http://filmicworlds.com/blog/minimal-color-grading-tools/
pub fn color_exposure(linear: Vector3, f_stop: FStop) -> Vector3 {
    // todo the powi should take exposure value if ev stop
    linear * 2.0f32.powi(f_stop.step)
}

// gray should be 0.18 for linear and 0.5 for gamma
pub fn linear_contrast(srgb: Vector3, gray: f32, contrast: f32) -> Vector3 {
    (srgb - gray) * contrast + gray
}

// gray should be 0.18 for linear and 0.5 for gamma
pub fn log_contrast(srgb: Vector3, gray: f32, contrast: f32) -> Vector3 {
    let log_rgb = Vector3 {
        value: log2((srgb + f32::EPSILON).value),
    };

    let log_gray = Vector3 {
        value: log2(Vector3::broadcast(gray).value),
    };

    let adjusted = log_gray + (log_rgb - log_gray) * contrast;

    Vector3 {
        value: exp2(adjusted.value) - f32::EPSILON,
    }
}

pub fn curve(
    color: Vector3,
    shadow_gamma: Vector3,
    midpoint: Vector3,
    highlight_scale: Vector3,
) -> Vector3 {
    let d = Vector3 {
        value: pow(color.value, shadow_gamma.value)
            * rcp(pow(midpoint.value, (shadow_gamma - 1.0).value)),
    };

    let l = highlight_scale * (color - midpoint) + midpoint;

    let mask = le(color.value, midpoint.value);

    Vector3 {
        value: select(d.value, l.value, mask),
    }
}

pub fn vibrance(srgb: Vector3, luminance: Vector3, balance: Vector3, vibrance: f32) -> Vector3 {
    // let luma = srgb_compute_luminance(srgb);
    //
    // let max_color = component_max(srgb.value);
    // let min_color = component_min(srgb.value);

    let luma = Vector3::broadcast(srgb_compute_luminance(srgb));
    let color_saturation = saturate(srgb, balance);

    let coeff_vibrance = balance * vibrance;
    let color = lerp(
        luma.value,
        srgb.value,
        (Vector3::broadcast(1.0)
            + (coeff_vibrance
                * (Vector3::broadcast(1.0)
                    - (Vector3 {
                        value: signum(coeff_vibrance.value),
                    }) * color_saturation)))
            .value,
    );

    todo!()
}
