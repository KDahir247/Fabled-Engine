use crate::camera::{Orthographic, Perspective};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Projection {
    Orthographic(Orthographic),
    Perspective(Perspective),
}


#[cfg(test)]
mod projection_test {
    // no test needed for projection, since it very primitive
}
