use crate::camera::{Orthographic, Perspective};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub enum Projection {
    Orthographic(Orthographic),
    Perspective(Perspective),
}


#[cfg(test)]
mod projection_test {
    // no test needed for projection, since it very primitive
}
