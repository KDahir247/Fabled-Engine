#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Orthographic {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            right: 1.0,
            left: 0.0,
            top: 1.0,
            bottom: 0.0,
        }
    }
}

impl Orthographic {
    pub fn new(right: f32, left: f32, top: f32, bottom: f32) -> Self {
        Self {
            right,
            left,
            top,
            bottom,
        }
    }
}

#[cfg(test)]
mod orthographic_test {
    // no test needed for orthographic, since it very primitive
}
