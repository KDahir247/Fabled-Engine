mod gamut_op;

pub use gamut_op::*;

use crate::color::oklab_to_srgb;
use fabled_math::vector_math::{component_max, component_min, dot, ge, pow, rcp, select};
use fabled_math::{Bool3, Swizzles4, Vector2, Vector3, Vector4};

pub(crate) fn compute_max_saturation(a: f32, b: f32) -> f32 {
    let mut w_lms = Vector3::ZERO;

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

            w_lms = Vector3::set(4.0767416621, -3.3077115913, 0.2309699292);
        } else if ((1.81444104 * a) - (1.19445276 * b)) > 1.0 {
            k0 = 0.73956515;
            k1 = -0.45954404;
            k2 = 0.08285427;
            k3 = 0.12541070;
            k4 = 0.14503204;

            w_lms = Vector3::set(1.2684380046, 2.6097574011, -0.3413193965);
        } else {
            k0 = 1.35733652;
            k1 = -0.00915799;
            k2 = -1.15130210;
            k3 = -0.50559606;
            k4 = 0.00692167;

            w_lms = Vector3::set(-0.0041960863, -0.7034186147, 1.7076147010);
        }

        k0 + (k1 * a) + (k2 * b) + (k3 * a) * a + (k4 * a) * b
    };


    let k_lms = Vector3::set(
        (0.3963377774 * a) + (0.2158037573 * b),
        -(0.1055613458 * a) - (0.0638541728 * b),
        -(0.0894841775 * a) - (1.2914855480 * b),
    );

    let res = {
        let lms_ = k_lms * s + Vector3::ONE;

        let lms = lms_ * lms_ * lms_;

        let lms_ds = (k_lms * 3.0) * (lms_ * lms_);

        let lms_ds2 = (k_lms * 6.0) * (k_lms * lms_);

        let f = dot(w_lms.value, lms.value);
        let f1 = dot(w_lms.value, lms_ds.value);
        let f2 = dot(w_lms.value, lms_ds2.value);

        let neg_half_f = -0.5 * f;
        let denominator = (f1 * f1) + (neg_half_f * f2);

        lms.z() - f * f1 / denominator
    };

    res
}

pub(crate) fn find_cusp(a: f32, b: f32) -> Vector2 {
    let s_cusp = compute_max_saturation(a, b);

    let rgb_at_max_lightness = oklab_to_srgb(Vector3::set(1.0, s_cusp * a, s_cusp * b));

    let rcp_max_channel = component_max(rgb_at_max_lightness.value).recip();

    let l_cusp = rcp_max_channel.cbrt();
    let c_cusp = l_cusp * s_cusp;

    Vector2::set(l_cusp, c_cusp)
}

pub(crate) fn find_gamut_intersection(lab: Vector3, c1: f32, l0: f32) -> f32 {
    let cusp = find_cusp(lab.y(), lab.z());

    let l = cusp.x();
    let c = cusp.y();

    let mut t = 0.0;

    let dl = lab.x() - l0;
    let rhs = l - l0;
    let intermediate = l0 - lab.x();

    // We will keep the conditional branch here since we don't want to calculate
    // everything in the else block and select the correct result. Conditional
    // will be faster.
    if ((dl * c) - (rhs * c1)) <= 0.0 {
        let denometer = (c1 * l) + (c * intermediate);
        t = (c * l0) / denometer;
    } else {
        let a = l0 - 1.0;
        let b = l - 1.0;

        let denometer = (c1 * b) + (c * intermediate);
        t = (c * a) / denometer;

        {
            let k_lms = Vector3::set(
                (0.3963377774 * a) + (0.2158037573 * b),
                -(0.1055613458 * a) - (0.0638541728 * b),
                -(0.0894841775 * a) - (1.2914855480 * b),
            );

            let lms_dt = k_lms * c1 + dl;

            {
                let d = 1.0 - t;
                let l = (l0 * d) + (t * lab.x());
                let c = t * c1;

                let lms_ = k_lms * c + l;

                let lms = lms_ * lms_ * lms_;

                let lmsdt = (lms_dt * 3.0) * (lms_ * lms_);

                let lmsdt2 = (lms_dt * 6.0) * (lms_dt * lms_);

                const RED_VECTOR: Vector3 = Vector3::set(4.0767416621, -3.3077115913, 0.2309699292);

                const GREEN_VECTOR: Vector3 =
                    Vector3::set(-1.2684380046, 2.6097574011, -0.3413193965);

                const BLUE_VECTOR: Vector3 =
                    Vector3::set(-0.0041960863, -0.7034186147, 1.7076147010);

                let rgb = Vector3::set(
                    dot(RED_VECTOR.value, lms.value) - 1.0,
                    dot(GREEN_VECTOR.value, lms.value) - 1.0,
                    dot(BLUE_VECTOR.value, lms.value) - 1.0,
                );

                let rgb_1 = Vector3::set(
                    dot(RED_VECTOR.value, lmsdt.value),
                    dot(GREEN_VECTOR.value, lmsdt.value),
                    dot(BLUE_VECTOR.value, lmsdt.value),
                );

                let rgb_2 = Vector3::set(
                    dot(RED_VECTOR.value, lmsdt2.value),
                    dot(GREEN_VECTOR.value, lmsdt2.value),
                    dot(BLUE_VECTOR.value, lmsdt2.value),
                );

                let rgb_1_half = rgb * 0.5;
                let u_rgb_denominator = (rgb_1 * rgb_1) - (rgb_1_half * rgb_2);

                let u_rgb = rgb_1 / u_rgb_denominator;
                let t_rgb = -rgb * u_rgb;

                let t_rgb = select(
                    t_rgb.value,
                    Vector3::broadcast(f32::MAX).value,
                    ge(u_rgb.value, Vector3::broadcast(0.0).value),
                );

                t += component_min(t_rgb)
            }
        }
    }
    t
}


pub(crate) fn aces_compression_internal(
    distance: Vector3,
    limit: Vector3,
    threshold: Vector4,
) -> Vector3 {
    let power = Vector3::broadcast(threshold.w());
    let rcp_power = rcp(power.value);

    let threshold = threshold.xyz();

    let limit_min_thresh = limit - threshold;
    let dist_min_thresh = distance - threshold;

    let s_intermediate = Vector3 {
        value: pow(
            ((Vector3::ONE - threshold) / limit_min_thresh).value,
            -power.value,
        ),
    };

    let s_denominator = Vector3 {
        value: pow((s_intermediate - 1.0).value, rcp_power),
    };

    let s = limit_min_thresh / s_denominator;

    let s_rcp = Vector3 {
        value: rcp(s.value),
    };

    let c_intermediate = Vector3 {
        value: pow((dist_min_thresh * s_rcp).value, power.value),
    };

    let c_denominator = Vector3 {
        value: pow((Vector3::ONE + c_intermediate).value, rcp_power),
    };

    let c_distance = threshold + s * (dist_min_thresh * s_rcp) / c_denominator;

    let compression_mask =
        Bool3::broadcast((distance < threshold) || (limit < Vector3::broadcast(1.0001)));

    Vector3 {
        value: select(distance.value, c_distance.value, compression_mask.value),
    }
}

pub(crate) fn aces_uncompressed_internal(
    distance: Vector3,
    limit: Vector3,
    threshold: Vector4,
) -> Vector3 {
    let power = Vector3::broadcast(threshold.w());
    let rcp_power = rcp(power.value);

    let threshold = threshold.xyz();

    let limit_min_thresh = limit - threshold;
    let dist_min_thresh = distance - threshold;

    let s_intermediate = Vector3 {
        value: pow(
            ((Vector3::ONE - threshold) / limit_min_thresh).value,
            -power.value,
        ),
    };

    let s_denominator = Vector3 {
        value: pow((s_intermediate - 1.0).value, rcp_power),
    };

    let s = limit_min_thresh / s_denominator;

    let s_rcp = Vector3 {
        value: rcp(s.value),
    };


    let c_intermediate = Vector3 {
        value: pow((dist_min_thresh * s_rcp).value, power.value),
    };

    let c = Vector3 {
        value: pow(-(c_intermediate / c_intermediate - 1.0).value, rcp_power),
    };

    let c_distance = threshold + s * c;

    let uncompressed_mask = Bool3::broadcast(
        (distance < threshold)
            || (limit < Vector3::broadcast(1.0001))
            || distance > (threshold + s),
    );

    Vector3 {
        value: select(distance.value, c_distance.value, uncompressed_mask.value),
    }
}
