use crate::camera::{Orthographic, Perspective};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Projection {
    Orthographic(Orthographic),
    Perspective(Perspective),
}
