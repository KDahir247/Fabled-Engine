use fabled_math::vector_math::{ge, log10, lt, pow, select};
use fabled_math::Vector3;

// HDR transfer function param
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

// linear to srgb
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


    let mask = ge(linear.value, Vector3::broadcast(0.0031308).value);

    let gamma = select(c.value, d.value, mask);

    Vector3 { value: gamma }
}


// srgb to linear
pub fn eotf_s_rgb(s_rgb: Vector3) -> Vector3 {
    let a = s_rgb + 0.055;

    let b = a * RCP_LINEAR_FALLOFF;
    let d = s_rgb * RCP_FALLOFF;

    let c = pow(b.value, Vector3::broadcast(2.4).value);

    let mask = ge(s_rgb.value, Vector3::broadcast(0.04045).value);

    let linear = select(c, d.value, mask);

    Vector3 { value: linear }
}

// linear to rec709
pub fn oetf_rec709(linear: Vector3) -> Vector3 {
    let b_ = Vector3 {
        value: pow(linear.value, Vector3::broadcast(0.45).value),
    };

    let a = linear * 4.5;
    let b = (b_ * 1.0993) - 0.0993;

    let mask = lt(linear.value, Vector3::broadcast(0.0181).value);

    Vector3 {
        value: select(a.value, b.value, mask),
    }
}

pub fn eotf_rec709(rec709: Vector3) -> Vector3 {
    const RCP_4_5: f32 = 1.0 / 4.5;
    const RCP_1: f32 = 1.0 / 1.0993;
    const RCP_4_5_TH: f32 = 1.0 / 0.45;

    let a = rec709 * RCP_4_5;

    let b = (rec709 + 0.0993) * RCP_1;

    let c = Vector3 {
        value: pow(b.value, Vector3::broadcast(RCP_4_5_TH).value),
    };

    let mask = lt(rec709.value, Vector3::broadcast(0.018).value);

    Vector3 {
        value: select(a.value, c.value, mask),
    }
}

// linear to pq
pub fn oetf_pq(linear_val: Vector3) -> Vector3 {
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
pub fn eotf_pq(pq_val: Vector3) -> Vector3 {
    let a = Vector3::broadcast(1.0 / PQ_M2);

    let e = Vector3 {
        value: pow(pq_val.value, a.value),
    };

    let m = e * PQ_C3;

    let t = Vector3::broadcast(PQ_C2) - m;
    let n = e - PQ_C1;

    let mask = ge(n.value, Vector3::ZERO.value);

    let clamped_n = Vector3 {
        value: select(n.value, Vector3::ZERO.value, mask),
    };

    let l = Vector3 {
        value: pow((clamped_n / t).value, Vector3::broadcast(1.0 / PQ_M1).value),
    };

    let linear = l * PQ_MAX;

    linear
}

pub fn linear_to_log_c(linear: Vector3) -> Vector3 {
    const A: f32 = 5.555556;
    const B: f32 = 0.047996;
    const C: f32 = 0.244161;
    const D: f32 = 0.386036;

    Vector3 {
        value: log10((linear * A + B).value),
    } * C
        + D
}

pub fn log_c_to_linear(log: Vector3) -> Vector3 {
    const A: f32 = 0.179999;
    const B: f32 = 0.047996;
    const C: f32 = 4.095658;
    const D: f32 = 0.386036;


    Vector3 {
        value: pow(Vector3::broadcast(10.0).value, ((log - D) * C).value),
    } * A
}
