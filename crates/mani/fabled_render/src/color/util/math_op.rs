pub fn saturate(val: f32) -> f32 {
    let rhs = val + 1.0;
    let lhs = val - 1.0;

    let temp = rhs - lhs.abs();
    (temp + temp.abs()) * 0.25
}
