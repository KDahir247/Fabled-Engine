pub fn acos(x: f32) -> f32 {
    let negate = ((x < 0.0) as i32) as f32;
    let x = x.abs();
    let mut ret = -0.0187293;
    ret *= x;
    ret += 0.0742610;
    ret *= x;
    ret -= 0.2121144;
    ret *= x;
    ret += 1.5707288;
    ret *= (1.0 - x).sqrt();
    ret = ret - 2.0 * negate * ret;

    negate * std::f32::consts::PI + ret
}
