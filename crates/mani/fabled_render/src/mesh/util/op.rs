// Don't use this.
pub fn sqrt_safe(value: f32) -> f32 {
    let mut temp: f32 = 0.0;
    let mut sqrt: f32 = value / 2.;
    let thresh = 0.000001;

    while (sqrt - temp).abs() > thresh {
        temp = sqrt;
        sqrt = (value / temp + temp) * 0.5;
    }
    sqrt
}
