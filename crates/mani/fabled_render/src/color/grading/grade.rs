use crate::camera::FStop;
use crate::color::srgb_compute_luminance;
use fabled_math::vector_math::{
    abs, clamp, dot, exp2, le, lerp, log2, max, pow, rcp, select, sign, signum, step,
};
use fabled_math::{Swizzles4, Vector3, Vector4};

// https://www.shadertoy.com/view/ss23DD
// from FMS_cat
pub fn apply_lift_gamma_gain(a: Vector3, lift: Vector4, gamma: Vector4, gain: Vector4) -> Vector3 {
    // since srgb and ACEScct have closely the same luminance this should be
    // sufficient for both.
    let luminance = Vector3::set(0.21263682, 0.71518298, 0.0721802);

    let liftt = -(Vector4 {
        value: pow(-(lift - 1.0).value, log2((gain + 1.0).value)),
    } - 1.0);

    let mut gammat = gamma - Vector4::set(0.0, 0.0, 0.0, dot(luminance.value, gamma.xyz().value));
    let gammat_temp = Vector4 {
        value: abs(gammat.value),
    } * 4.0
        + 1.0;

    gammat = Vector4 {
        value: lerp(
            gammat_temp.value,
            rcp(gammat_temp.value),
            step(Vector4::ZERO.value, gammat.value),
        ),
    };

    let mut col = a;
    let mut luma = dot(luminance.value, col.value);

    col = Vector3 {
        value: pow(col.value, gammat.xyz().value),
    };
    col *= Vector3 {
        value: pow(gain.xyz().value, gammat.xyz().value),
    };

    col = Vector3 {
        value: max(
            lerp((liftt.xyz() * 2.0).value, Vector3::ONE.value, col.value),
            Vector3::ZERO.value,
        ),
    };

    luma = luma.powf(gammat.w());
    luma *= gain.w().powf(gammat.w());

    let lerp_a = (2.0 * liftt.w()) * (1.0 - luma) + 1.0 * luma;

    luma = lerp_a.max(0.0);

    col += luma - dot(luminance.value, col.value);

    return col;
}

pub fn apply_asc_cdl(a: Vector3, slope: Vector3, offset: Vector3, power: Vector3) -> Vector3 {
    Vector3 {
        value: pow((a * slope + offset).value, power.value),
    }
}

pub fn desaturate(a: Vector3, factor: Vector3) -> Vector3 {
    let max_luminance = srgb_compute_luminance(a);

    let grey = Vector3::broadcast(max_luminance);

    Vector3 {
        value: lerp(a.value, grey.value, factor.value),
    }
}

pub fn saturation(a: Vector3, factor: Vector3) -> Vector3 {
    let max_luminance = srgb_compute_luminance(a);

    let luminance = Vector3::broadcast(max_luminance);

    let diff = a - luminance;
    let saturate = luminance * factor;

    Vector3 {
        value: clamp(
            (saturate + diff).value,
            Vector3::ZERO.value,
            Vector3::ONE.value,
        ),
    }
}

// http://filmicworlds.com/blog/minimal-color-grading-tools/
pub fn color_exposure(a: Vector3, f_stop: FStop) -> Vector3 {
    // todo the powi should take exposure value if ev stop
    a * 2.0f32.powi(f_stop.step)
}

// gray should be 0.18 for linear and 0.5 for gamma
pub fn linear_contrast(a: Vector3, gray: f32, contrast: f32) -> Vector3 {
    (a - gray) * contrast + gray
}

// gray should be 0.18 for linear and 0.5 for gamma
pub fn log_contrast(a: Vector3, gray: f32, contrast: f32) -> Vector3 {
    let log_rgb = Vector3 {
        value: log2((a + f32::EPSILON).value),
    };

    let log_gray = Vector3 {
        value: log2(Vector3::broadcast(gray).value),
    };

    let adjusted = log_gray + (log_rgb - log_gray) * contrast;

    Vector3 {
        value: exp2(adjusted.value),
    } - f32::EPSILON
}

pub fn curve(
    a: Vector3,
    shadow_gamma: Vector3,
    midpoint: Vector3,
    highlight_scale: Vector3,
) -> Vector3 {
    let d = Vector3 {
        value: pow(a.value, shadow_gamma.value)
            * rcp(pow(midpoint.value, (shadow_gamma - 1.0).value)),
    };

    let l = highlight_scale * (a - midpoint) + midpoint;

    let mask = le(a.value, midpoint.value);

    Vector3 {
        value: select(d.value, l.value, mask),
    }
}

pub fn color_filter(a: Vector3, b: Vector3) -> Vector3 {
    a * b
}


// pub fn vibrance(srgb: Vector3, luminance: Vector3, balance: Vector3,
// vibrance: f32) -> Vector3 {     // let luma = srgb_compute_luminance(srgb);
//     //
//     // let max_color = component_max(srgb.value);
//     // let min_color = component_min(srgb.value);
//
//     let luma = Vector3::broadcast(srgb_compute_luminance(srgb));
//     let color_saturation = saturation(srgb, balance);
//
//     let coeff_vibrance = balance * vibrance;
//     let color = lerp(
//         luma.value,
//         srgb.value,
//         (Vector3::broadcast(1.0)
//             + (coeff_vibrance
//                 * (Vector3::broadcast(1.0)
//                     - (Vector3 { value: signum(coeff_vibrance.value),
//                     }) * color_saturation)))
//             .value,
//     );
//
//     todo!()
// }


// value of channel should be greater than or equal to -2 or less than or equal
// to 2.0;
pub fn channel_mixer(a: Vector3, red: Vector3, green: Vector3, blue: Vector3) -> Vector3 {
    Vector3::set(
        dot(a.value, red.value),
        dot(a.value, green.value),
        dot(a.value, blue.value),
    )
}
