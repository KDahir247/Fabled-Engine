use crate::color::{eotf_s_rgb, get_cs, oetf_s_rgb, oklab_to_srgb, srgb_to_oklab, toe, toe_inv};
use fabled_math::{approximate_equal, Vector3};

pub fn okhsl_to_srgb(hsl: Vector3) -> Vector3 {
    if approximate_equal(hsl.z(), 0.0, f32::EPSILON) {
        return Vector3::ONE;
    } else if approximate_equal(hsl.z(), 1.0, f32::EPSILON) {
        return Vector3::ONE;
    }

    let (b, a) = (2.0 * std::f32::consts::PI * hsl.x()).sin_cos();
    let l = toe_inv(hsl.z());

    let cs = get_cs(l, a, b);

    let mid = 0.8;
    let mid_inv = 1.25;

    let mut c = 0.0;
    let mut t = 0.0;
    let mut k_0 = 0.0;
    let mut k_1 = 0.0;
    let mut k_2 = 0.0;

    if hsl.y() < mid {
        t = mid_inv * hsl.y();

        k_1 = mid * cs.x();
        k_2 = 1.0 - k_1 / cs.y();

        c = t * k_1 / (1.0 - k_2 * t);
    } else {
        t = (hsl.y() - mid) / (1.0 - mid);

        k_0 = cs.y();
        k_1 = (1.0 - mid) * cs.y() * cs.y() * mid_inv * mid_inv / cs.x();
        k_2 = (1.0 - k_1) / (cs.z() - cs.y());

        c = k_0 + t * k_1 / (1.0 - k_2 * t);
    }

    let oklab = Vector3::set(l, c * a, c * b);
    let srgb = oklab_to_srgb(oklab);

    oetf_s_rgb(srgb)
}


pub fn srgb_to_okhsl(srgb: Vector3) -> Vector3 {
    let linear = eotf_s_rgb(srgb);

    let lab = srgb_to_oklab(linear);

    let c = ((lab.y() * lab.y()) + (lab.z() * lab.z())).sqrt();

    let a = lab.y() / c;
    let b = lab.z() / c;

    let l = lab.x();

    let h = 0.5 + 0.5 * -lab.z().atan2(-lab.y()) * std::f32::consts::FRAC_1_PI;

    let cs = get_cs(l, a, b);

    let mid = 0.8;
    let mid_inv = 1.25;

    let mut s = 0.0;

    if c < cs.y() {
        let k_1 = mid * cs.x();
        let k_2 = 1.0 - k_1 / cs.y();

        let t = c / (k_1 + k_2 * c);
        s = t * mid;
    } else {
        let k_0 = cs.y();
        let k_1 = (1.0 - mid) * cs.y() * cs.y() * mid_inv * mid_inv / cs.x();
        let k_2 = 1.0 - k_1 / (cs.z() - cs.y());

        let t = (c - k_0) / (k_1 + k_2 * (c - k_0));

        s = mid + (1.0 - mid) * t;
    }

    let l = toe(l);

    Vector3::set(h, s, l)
}
