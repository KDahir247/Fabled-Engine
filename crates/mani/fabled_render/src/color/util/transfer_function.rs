use fabled_math::vector_math::{ge, pow, select};
use fabled_math::Vector3;

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


pub fn oetf_s_rgb(linear: Vector3) -> Vector3 {
    let a = Vector3 {
        value: pow(
            linear.value,
            Vector3::broadcast(SRGB_POW_TRANSFER_FUNCTION).value,
        ),
    };

    let b = a * 1.055;
    let c = b - 0.055;

    let d = linear * 12.92;


    let mask = ge(linear.value, Vector3::broadcast(0.0031308).value);

    let gamma = select(c.value, d.value, mask);

    Vector3 { value: gamma }
}


pub fn eotf_s_rgb(s_rgb: Vector3) -> Vector3 {
    let a = s_rgb + 0.055;

    let b = a * RCP_LINEAR_FALLOFF;
    let d = s_rgb * RCP_FALLOFF;

    let c = pow(b.value, Vector3::broadcast(2.4).value);

    let mask = ge(s_rgb.value, Vector3::broadcast(0.04045).value);

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
    let mask = ge(n.value, Vector3::broadcast(0.0).value);

    let clamped_n = Vector3 {
        value: select(n.value, ZERO_VEC3.value, mask),
    };

    let l = Vector3 {
        value: pow((clamped_n / t).value, Vector3::broadcast(1.0 / PQ_M1).value),
    };

    let linear = l * PQ_MAX;

    linear
}
