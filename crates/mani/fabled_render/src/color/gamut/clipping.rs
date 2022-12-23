use crate::color::{find_gamut_intersection, oklab_to_srgb, srgb_to_oklab};
use fabled_math::vector_math::{gt, lt};
use fabled_math::{Bool3, Vector3};

pub fn gamut_clip_adaptive_l0_0_5(rgb: Vector3, alpha: Option<f32>) -> Vector3 {
    let alpha = alpha.unwrap_or(0.05);

    let normalized_min = Bool3 {
        value: lt(rgb.value, Vector3::broadcast(1.0).value),
    };
    let normalized_max = Bool3 {
        value: gt(rgb.value, Vector3::broadcast(0.0).value),
    };

    if normalized_min.all() & normalized_max.all() {
        return rgb;
    }

    let lab = srgb_to_oklab(rgb);

    let l = lab.x();

    let length = ((lab.y() * lab.y()) + (lab.z() * lab.z()))
        .sqrt()
        .max(f32::EPSILON);

    let rcp_length = length.recip();

    let a_ = lab.y() * rcp_length;
    let b_ = lab.z() * rcp_length;

    let ld = l - 0.5;
    let ld_abs = ld.abs();
    let e1 = (0.5 + ld_abs) + (alpha * length);

    let c = ((e1 * e1) - (ld_abs + ld_abs)).sqrt();
    let l0 = 0.5 * (1.0 + ld.signum() * (e1 - c));

    let t = find_gamut_intersection(a_, b_, l, length, l0);

    let d = 1.0 - t;
    let l_clipped = (l0 * d) + (t * l);
    let c_clipped = t * length;

    let a = c_clipped * a_;
    let b = c_clipped * b_;

    oklab_to_srgb(Vector3::set(l_clipped, a, b))
}
