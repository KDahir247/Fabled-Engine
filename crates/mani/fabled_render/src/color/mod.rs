mod adaption;
mod component;
mod constant;
mod container;
mod conversion;
mod gamut;
mod grading;
mod space;
mod util;

pub use adaption::*;
pub use constant::*;
pub use container::*;
pub use conversion::*;
pub use gamut::*;
pub use grading::*;
pub use space::*;
use std::ops::{BitAnd, Div, Shl, Shr};
pub use util::*;


use fabled_math::vector_math::{cast, reduce_or, reduce_sum};
use fabled_math::{Vector3, Vector4};

// todo don't like the simd cast happening here. Got to create some container
//  for integer type.

pub fn rgb_to_hex(rgb: Vector3) -> usize {
    let shift_mask = cast::<f32, usize>(Vector3::set(16.0, 8.0, 0.0).value);
    let mul_mask = cast::<f32, usize>(Vector3::broadcast(255.0).value);
    let scaled_rgb = cast::<f32, usize>(rgb.value) * mul_mask;

    let hex_decimal_split = scaled_rgb.shl(shift_mask);

    reduce_or(hex_decimal_split)
}

pub fn rgba_to_hex(rgba: Vector4) -> usize {
    let shift_mask = cast::<f32, usize>(Vector4::set(24.0, 16.0, 8.0, 0.0).value);
    let mul_mask = cast::<f32, usize>(Vector4::broadcast(255.).value);
    let scaled_rgba = cast::<f32, usize>(rgba.value) * mul_mask;

    let hex_decimal_split = scaled_rgba.shl(shift_mask);

    reduce_sum(hex_decimal_split)
}

pub fn hex_to_rgb(hex: usize) -> Vector3 {
    let signed_hex = hex as isize;

    let scaled_mask = cast::<f32, usize>(Vector3::broadcast(255.0).value);
    let shift_mask = cast::<f32, usize>(Vector3::set(16.0, 8.0, 0.0).value);
    let hex_shift = cast::<f32, usize>(Vector3::broadcast(signed_hex as f32).value).shr(shift_mask);
    let scaled_rgb = hex_shift.bitand(scaled_mask);

    Vector3 {
        value: cast::<usize, f32>(scaled_rgb.div(scaled_mask)),
    }
}

pub fn hex_to_rgba(hex: usize) -> Vector4 {
    let signed_hex = hex as isize;

    let scaled_mask = cast::<f32, usize>(Vector4::broadcast(255.0).value);
    let shift_mask = cast::<f32, usize>(Vector4::set(24.0, 16.0, 8.0, 0.0).value);
    let hex_shift = cast::<f32, usize>(Vector4::broadcast(signed_hex as f32).value).shr(shift_mask);
    let scaled_rgba = hex_shift.bitand(scaled_mask);

    Vector4 {
        value: cast::<usize, f32>(scaled_rgba.div(scaled_mask)),
    }
}

pub fn create_rgb(r: usize, g: usize, b: usize) -> usize {
    let signed_r = r as isize;
    let signed_g = g as isize;
    let signed_b = b as isize;

    let rgb = Vector3::set(signed_r as f32, signed_g as f32, signed_b as f32);

    let shift_mask = cast::<f32, usize>(Vector3::set(16.0, 8.0, 0.0).value);
    let mul_mask = cast::<f32, usize>(Vector3::broadcast(255.0).value);

    let scaled_rgb = cast::<f32, usize>(rgb.value).bitand(mul_mask);

    let hex_decimal_split = scaled_rgb.shl(shift_mask);

    reduce_sum(hex_decimal_split)
}

pub fn create_rgba(r: usize, g: usize, b: usize, a: usize) -> usize {
    let signed_r = r as isize;
    let signed_g = g as isize;
    let signed_b = b as isize;
    let signed_a = a as isize;

    let rgba = Vector4::set(
        signed_r as f32,
        signed_g as f32,
        signed_b as f32,
        signed_a as f32,
    );
    let shift_mask = cast::<f32, usize>(Vector4::set(24.0, 16.0, 8.0, 0.0).value);
    let mul_mask = cast::<f32, usize>(Vector4::broadcast(255.0).value);
    let scaled_rgba = cast::<f32, usize>(rgba.value).bitand(mul_mask);
    let hex_decimal_split = scaled_rgba.shl(shift_mask);

    reduce_sum(hex_decimal_split)
}
