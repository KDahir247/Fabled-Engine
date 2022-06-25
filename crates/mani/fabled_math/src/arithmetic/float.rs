pub trait FloatExtension {
    fn saturate(&self) -> Self;
}


impl FloatExtension for f32 {
    fn saturate(&self) -> Self {
        0.0f32.max(1.0f32.min(*self))
    }
}

impl FloatExtension for f64 {
    fn saturate(&self) -> Self {
        0.0f64.max(1.0f64.min(*self))
    }
}
