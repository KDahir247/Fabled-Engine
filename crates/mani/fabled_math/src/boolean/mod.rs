mod bool2;
mod bool3;
mod bool4;

pub use bool2::*;
pub use bool3::*;
pub use bool4::*;


#[inline(always)]
pub fn approximate_equal(a: f32, b: f32, epsilon: f32) -> bool {
    let max = a.max(b);

    (a - b).abs() <= (max * epsilon)
}

#[inline(always)]
pub fn approximate_equal3(
    a: std::simd::f32x4,
    b: std::simd::f32x4,
    epsilon: std::simd::f32x4,
) -> Bool4 {
    let x_cmp = approximate_equal(a[0], b[0], epsilon[0]);
    let y_cmp = approximate_equal(a[1], b[1], epsilon[1]);
    let z_cmp = approximate_equal(a[2], b[2], epsilon[2]);
    let w_cmp = approximate_equal(a[3], b[3], epsilon[3]);

    Bool4::set(x_cmp, y_cmp, z_cmp, w_cmp)
}

#[inline(always)]
pub fn fast_conditional(val: i32, true_statement: i32, false_statement: i32) -> i32 {
    (val & true_statement) | (!val & false_statement)
}
