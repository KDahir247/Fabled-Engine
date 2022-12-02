use fabled_math::vector_math::{component_ge, pow, select};
use fabled_math::{Vector2, Vector3};

// HDR ICTCP transfer function param
const PQ_C1: f32 = 0.8359375;
const PQ_C2: f32 = 18.8515625;
const PQ_C3: f32 = 18.6875;
const PQ_M1: f32 = 0.159301758125;
const PQ_M2: f32 = 78.84375;
const PQ_MAX: f32 = 10000.0;


// SRGB transfer function param
const SRGB_POW_TRANSFER_FUNCTION: f32 = 1.0 / 2.4;

const RCP_FALLOFF: f32 = 1.0 / 12.92;
const RCP_LINEAR_FALLOFF: f32 = 1.0 / 1.055;


pub fn saturate(val: f32) -> f32 {
    let rhs = val + 1.0;
    let lhs = val - 1.0;

    let temp = rhs - lhs.abs();
    (temp + temp.abs()) * 0.25
}

pub fn oetf_s_rgb(linear: Vector3) -> Vector3 {
    let a = Vector3 {
        value: pow(
            linear.value,
            Vector3::broadcast(SRGB_POW_TRANSFER_FUNCTION).value,
        ),
    };

    let b = a * 1.055;
    let d = linear * 12.92;

    let c = b - 0.055;

    let mask = component_ge(linear.value, 0.0031308);

    let non_linear = select(c.value, d.value, mask);

    Vector3 { value: non_linear }
}


pub fn eotf_s_rgb(s_rgb: Vector3) -> Vector3 {
    let a = s_rgb + 0.055;

    let b = a * RCP_LINEAR_FALLOFF;
    let d = s_rgb * RCP_FALLOFF;

    let c = pow(b.value, Vector3::broadcast(2.4).value);

    let mask = component_ge(s_rgb.value, 0.04045);

    let linear = select(c, d.value, mask);

    Vector3 { value: linear }
}


pub fn pq_oetf(linear_val: Vector3) -> Vector3 {
    let l = linear_val / PQ_MAX;

    let lm1 = Vector3 {
        value: pow(l.value, Vector3::broadcast(PQ_M1).value),
    };

    let x = (lm1 * PQ_C2 + PQ_C1) / (lm1 * PQ_C3 + 1.0);

    let pq_val = Vector3 {
        value: pow(x.value, Vector3::broadcast(PQ_M2).value),
    };

    pq_val
}

// pq to linear
pub fn pq_eotf(pq_val: Vector3) -> Vector3 {
    let a = Vector3::broadcast(1.0 / PQ_M2);

    let m = Vector3 {
        value: pow(pq_val.value, a.value),
    } * PQ_C3;
    let t = -(m - PQ_C2);

    let n = Vector3 {
        value: pow(pq_val.value, a.value),
    } - PQ_C1;

    const ZERO_VEC3: Vector3 = Vector3::broadcast(0.0);
    let mask = component_ge(n.value, 0.0);

    let clamped_n = Vector3 {
        value: select(n.value, ZERO_VEC3.value, mask),
    };

    let l = Vector3 {
        value: pow((clamped_n / t).value, Vector3::broadcast(1.0 / PQ_M1).value),
    };

    let linear = l * PQ_MAX;

    linear
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
