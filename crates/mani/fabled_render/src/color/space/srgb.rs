use crate::color::{eotf_s_rgb, oetf_s_rgb};
use fabled_math::Vector3;


pub fn s_rgb_to_linear(s_rgb: Vector3) -> Vector3 {
    eotf_s_rgb(s_rgb)
}

pub fn linear_to_s_rgb(s_rgb: Vector3) -> Vector3 {
    oetf_s_rgb(s_rgb)
}
