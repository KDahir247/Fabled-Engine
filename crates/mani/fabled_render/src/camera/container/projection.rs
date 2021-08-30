use crate::camera::{Orthographic, Perspective};

//Lhs and Rhs, Direction (Y Up, Y Down).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Projection {
    Orthographic(Orthographic),
    Perspective(Perspective),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProjectionCoordinate {
    LeftHandCoordinate = -1,
    RightHandCoordinate = 1,
}

impl Default for ProjectionCoordinate {
    fn default() -> Self {
        Self::RightHandCoordinate
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum YAxis {
    Up,
    Down,
}

impl Default for YAxis {
    fn default() -> Self {
        Self::Up
    }
}
