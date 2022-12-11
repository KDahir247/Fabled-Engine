use crate::color::{
    eotf_s_rgb, find_cusp, hsv_to_hwb, hwb_to_hsv, oetf_s_rgb, oklab_to_srgb, srgb_to_oklab, to_st,
    toe, toe_inv,
};
use fabled_math::vector_math::component_max;
use fabled_math::Vector3;

pub fn okhsv_to_hwb(okhsv: Vector3) -> Vector3 {
    hsv_to_hwb(okhsv)
}

pub fn hwb_to_okhsv(hwb: Vector3) -> Vector3 {
    hwb_to_hsv(hwb)
}

pub fn okhsv_to_srgb(okhsv: Vector3) -> Vector3 {
    let hue = okhsv.x();
    let saturation = okhsv.y();
    let value = okhsv.z();

    let t = 2.0 * std::f32::consts::PI * hue;

    let (b_, a_) = t.sin_cos();

    let cups = find_cusp(a_, b_);
    let st_max = to_st(cups);

    let s_max = st_max.x();
    let t_max = st_max.y();

    let s_0 = 0.5;

    let k = 1.0 - s_0 / s_max;

    let l_v = 1.0 - saturation * s_0 / (s_0 + t_max - t_max * k * saturation);
    let c_v = saturation * t_max * s_0 / (s_0 + t_max - t_max * k * saturation);

    let l = value * l_v;
    let c = value * c_v;

    let l_vt = toe_inv(l_v);
    let c_vt = c_v * l_vt / l_v;

    let l_new = toe_inv(l);
    let c = c * l_new / l;
    let l = l_new;

    let oklab_scale = Vector3::set(l_vt, a_ * c_vt, b_ * c_vt);
    let rgb_scale = oklab_to_srgb(oklab_scale);

    let max_luminance = component_max(rgb_scale.value).max(0.0);

    let scale_l = (1.0 / max_luminance).cbrt();


    let l = l * scale_l;
    let c = c * scale_l;

    let oklab = Vector3::set(l, c * a_, c * b_);

    let rgb = oklab_to_srgb(oklab);

    oetf_s_rgb(rgb)
}


pub fn srgb_to_okhsv(srgb: Vector3) -> Vector3 {
    let oklab = srgb_to_oklab(eotf_s_rgb(srgb));

    let c = ((oklab.y() * oklab.y()) + (oklab.z() * oklab.z())).sqrt();
    let a_ = oklab.y() / c;
    let b_ = oklab.z() / c;

    let l = oklab.x();
    let h = 0.5 + 0.5 * -oklab.z().atan2(-oklab.y()) * std::f32::consts::FRAC_1_PI;

    let cusp = find_cusp(a_, b_);
    let st_max = to_st(cusp);
    let s_max = st_max.x();
    let t_max = st_max.y();
    let s_0 = 0.5;
    let k = 1.0 - s_0 / s_max;

    let t = t_max / (c + l * t_max);
    let l_v = t * l;
    let c_v = t * c;

    let l_vt = toe_inv(l_v);
    let c_vt = c_v * l_vt / l_v;

    let oklab_scale = Vector3::set(l_vt, a_ * c_vt, b_ * c_vt);
    let rgb_scale = oklab_to_srgb(oklab_scale);

    let max_luminance = component_max(rgb_scale.value).max(0.0);

    let scale_l = (1.0 / max_luminance).cbrt();

    let l = l * scale_l;
    let c = c * scale_l;

    let c = c * toe(l) / l;
    let l = toe(l);

    let v = l / l_v;
    let s = (s_0 + t_max) * c_v / ((t_max * s_0) + t_max * k * c_v);

    Vector3::set(h, s, v)
}
