use crate::color::{oklab_to_srgb, srgb_to_oklab};
use fabled_math::vector_math::{component_max, component_min, ge, gt, lt, select};
use fabled_math::{Bool3, Vector2, Vector3};

// todo move util function to mod.rs and constraint it to pub(crate)

pub fn compute_max_saturation(a: f32, b: f32) -> f32 {
    let mut wl = 0.0;
    let mut wm = 0.0;
    let mut ws = 0.0;

    let s = {
        let mut k0 = 0.0;
        let mut k1 = 0.0;
        let mut k2 = 0.0;
        let mut k3 = 0.0;
        let mut k4 = 0.0;

        if (-(1.88170328 * a) - (0.80936493 * b)) > 1.0 {
            k0 = 1.19086277;
            k1 = 1.76576728;
            k2 = 0.59662641;
            k3 = 0.75515197;
            k4 = 0.56771245;
            wl = 4.0767416621;
            wm = -3.3077115913;
            ws = 0.2309699292;
        } else if ((1.81444104 * a) - (1.19445276 * b)) > 1.0 {
            k0 = 0.73956515;
            k1 = -0.45954404;
            k2 = 0.08285427;
            k3 = 0.12541070;
            k4 = 0.14503204;
            wl = 1.2684380046;
            wm = 2.6097574011;
            ws = -0.3413193965;
        } else {
            k0 = 1.35733652;
            k1 = -0.00915799;
            k2 = -1.15130210;
            k3 = -0.50559606;
            k4 = 0.00692167;
            wl = -0.0041960863;
            wm = -0.7034186147;
            ws = 1.7076147010;
        }

        k0 + (k1 * a) + (k2 * b) + (k3 * a) * a + (k4 * a) * b
    };

    let k_l = (0.3963377774 * a) + (0.2158037573 * b);
    let k_m = -(0.1055613458 * a) - (0.0638541728 * b);
    let k_s = -(0.0894841775 * a) - (1.2914855480 * b);

    let res = {
        let l_ = 1.0 + s * k_l;
        let m_ = 1.0 + s * k_m;
        let s_ = 1.0 + s * k_s;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let l_ds = (3.0 * k_l) * (l_ * l_);
        let m_ds = (3.0 * k_m) * (m_ * m_);
        let s_ds = (3.0 * k_s) * (s_ * s_);

        let l_ds2 = (6.0 * k_l) * (k_l * l_);
        let m_ds2 = (6.0 * k_m) * (k_m * m_);
        let s_ds2 = (6.0 * k_s) * (k_s * s_);

        let f = (wl * l) + (wm * m) + (ws * s);
        let f1 = (wl * l_ds) + (wm * m_ds) + (ws * s_ds);
        let f2 = (wl * l_ds2) + (wm * m_ds2) + (ws * s_ds2);

        let neg_half_f = -0.5 * f;
        let denominator = (f1 * f1) + (neg_half_f * f2);
        s - f * f1 / denominator
    };

    res
}

pub fn find_cusp(a: f32, b: f32) -> Vector2 {
    let s_cusp = compute_max_saturation(a, b);

    let rgb_at_max = oklab_to_srgb(Vector3::set(1.0, s_cusp * a, s_cusp * b));

    let l_cusp = (1.0 / component_max(rgb_at_max.value)).cbrt();
    let c_cusp = l_cusp * s_cusp;

    Vector2::set(l_cusp, c_cusp)
}

pub fn find_gamut_intersection(a: f32, b: f32, l1: f32, c1: f32, l0: f32) -> f32 {
    let cusp = find_cusp(a, b);

    let mut t = 0.0;

    let dl = l1 - l0;
    let rhs = cusp.x() - l0;
    let intermediate = l0 - l1;

    if ((dl * cusp.y()) - (rhs * c1)) <= 0.0 {
        t = (cusp.y() * l0) / ((c1 * cusp.x()) + (cusp.y() * intermediate));
    } else {
        let a = l0 - 1.0;
        let b = cusp.x() - 1.0;

        t = (cusp.y() * a) / ((c1 * b) + (cusp.y() * intermediate));

        {
            let k_l = (0.3963377774 * a) + (0.2158037573 * b);
            let k_m = -(0.1055613458 * a) - (0.0638541728 * b);
            let k_s = -(0.0894841775 * a) - (1.2914855480 * b);

            let l_dt = dl + c1 * k_l;
            let m_dt = dl + c1 * k_m;
            let s_dt = dl + c1 * k_s;

            {
                let d = 1.0 - t;
                let l = (l0 * d) + (t * l1);
                let c = t * c1;

                let l_ = l + c * k_l;
                let m_ = l + c * k_m;
                let s_ = l + c * k_s;

                let l = l_ * l_ * l_;
                let m = m_ * m_ * m_;
                let s = s_ * s_ * s_;

                let ldt = (3.0 * l_dt) * (l_ * l_);
                let mdt = (3.0 * m_dt) * (m_ * m_);
                let sdt = (3.0 * s_dt) * (s_ * s_);

                let ldt2 = (6.0 * l_dt) * (l_dt * l_);
                let mdt2 = (6.0 * m_dt) * (m_dt * m_);
                let sdt2 = (6.0 * s_dt) * (s_dt * s_);

                let r = (4.0767416621 * l) - (3.3077115913 * m) + (0.2309699292 * s) - 1.0;
                let r1 = (4.0767416621 * ldt) - (3.3077115913 * mdt) + (0.2309699292 * sdt);
                let r2 = (4.0767416621 * ldt2) - (3.3077115913 * mdt2) + (0.2309699292 * sdt2);


                let g = -(1.2684380046 * l) + (2.6097574011 * m) - (0.3413193965 * s) - 1.0;
                let g1 = -(1.2684380046 * ldt) + (2.6097574011 * mdt) - (0.3413193965 * sdt);
                let g2 = -(1.2684380046 * ldt2) + (2.6097574011 * mdt2) - (0.3413193965 * sdt2);


                let b = -(0.0041960863 * l) - (0.7034186147 * m) + (1.7076147010 * s) - 1.0;
                let b1 = -(0.0041960863 * ldt) - (0.7034186147 * mdt) + (1.7076147010 * sdt);
                let b2 = -(0.0041960863 * ldt2) - (0.7034186147 * mdt2) + (1.7076147010 * sdt2);

                let rgb1_half = Vector3::set(0.5 * r, 0.5 * g, 0.5 * b);

                let u_rgb = Vector3::set(
                    r1 / ((r1 * r1) - (rgb1_half.x() * r2)),
                    g1 / ((g1 * g1) - (rgb1_half.y() * g2)),
                    b1 / ((b1 * b1) - (rgb1_half.z() * b2)),
                );

                let t_rgb = Vector3::set(-r * u_rgb.x(), -g * u_rgb.y(), -b * u_rgb.z());

                let max_vector3 = Vector3::broadcast(f32::MAX);


                let mask = ge(u_rgb.value, Vector3::broadcast(0.0).value);

                let t_rgb = select(t_rgb.value, max_vector3.value, mask);

                t += component_min(t_rgb)
            }
        }
    }
    t
}


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
