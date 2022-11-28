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


// linear to pq
pub fn pq_oetf(linear_val: Vector3) -> Vector3 {
    todo!()
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
