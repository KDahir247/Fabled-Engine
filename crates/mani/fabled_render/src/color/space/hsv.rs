use fabled_math::vector_math::{abs, fract, lerp, saturate, step};
use fabled_math::{Swizzles3, Swizzles4, Vector3, Vector4};

pub fn rgb_to_hsv(rgb: Vector3) -> Vector3 {
    let p_src = Vector4::set(rgb.z(), rgb.y(), -1.0, 0.66666666666666666666666666666667);
    let p_dst = Vector4::set(rgb.y(), rgb.z(), 0.0, -0.33333333333333333333333333333333);

    let p_step = step(
        Vector4::broadcast(rgb.z()).value,
        Vector4::broadcast(rgb.y()).value,
    );

    let p = Vector4 {
        value: lerp(p_src.value, p_dst.value, p_step),
    };

    let q_src = Vector4::set(p.x(), p.y(), p.w(), rgb.x());
    let q_dst = Vector4::set(rgb.x(), p.y(), p.z(), p.x());

    let q_step = step(
        Vector4::broadcast(p.x()).value,
        Vector4::broadcast(rgb.x()).value,
    );

    let q = Vector4 {
        value: lerp(q_src.value, q_dst.value, q_step),
    };

    let d = q.x() - q.w().min(q.y());
    let r_denometer = (d + d) + (d + d) + (d + d) + f32::EPSILON;
    let g_denometer = q.x() + f32::EPSILON;

    let r = (q.z() + (q.w() - q.y()) / r_denometer).abs();
    let g = d / g_denometer;
    let b = q.x();

    Vector3::set(r, g, b)
}


pub fn hsv_to_rgb(hsv: Vector3) -> Vector3 {
    const K: Vector4 = Vector4::set(
        1.0,
        0.66666666666666666666666666666667,
        0.33333333333333333333333333333333,
        3.0,
    );

    let d = Vector3 {
        value: fract((hsv.xxx() + K.xyz()).value),
    };

    let d_mul_6 = (d + d) + (d + d) + (d + d);

    let p = Vector3 {
        value: abs((d_mul_6 - 3.0).value),
    };

    let rgb_step = Vector3::broadcast(hsv.y());

    let rgb = Vector3 {
        value: lerp(
            K.xxx().value,
            saturate(-(K.xxx() - p).value),
            rgb_step.value,
        ),
    } * hsv.z();

    rgb
}


pub fn hsv_to_hwb(hsv: Vector3) -> Vector3 {
    let hue = hsv.x();
    let whiteness = (1.0 - hsv.y()) * hsv.z();
    let blackness = 1.0 - hsv.z();


    Vector3::set(hue, whiteness, blackness)
}


pub fn hwb_to_hsv(hwb: Vector3) -> Vector3 {
    let hue = hwb.x();
    let saturation = 1.0 - (hwb.y() / (1.0 - hwb.z()));
    let value = 1.0 - hwb.z();

    Vector3::set(hue, saturation, value)
}
