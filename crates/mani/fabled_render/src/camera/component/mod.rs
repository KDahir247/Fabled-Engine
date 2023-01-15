pub use aperture::*;
pub use aspect_ratio::*;
pub use clipping_plane::*;
pub use f_stop::*;
use fabled_component::{Unique, Untracked};
use fabled_math::Matrix4x4;
pub use fov::*;
pub use iso_speed::*;
pub use oblique::*;
pub use shutter::*;
pub use viewport::*;

mod aperture;
mod aspect_ratio;
mod clipping_plane;
mod f_stop;
mod fov;
mod iso_speed;
mod oblique;
mod shutter;
mod viewport;

// Camera component ECS
// Entity
// FStop -> ISO Speed -> Shutter -> ClippingPlane -> Viewport

// Perspective
// FStop -> ISO Speed -> Shutter -> ClippingPlane -> Viewport -> AspectRatio ->
// Fov -> Aperture


// Orthographic
// FStop -> ISO Speed -> Shutter -> ClippingPlane -> Viewport -> Orientation ->
// Oblique (optional)


// The active camera's projection matrix.
// This is a resource since there can only be one camera rendered to a screen
#[derive(Copy, Clone, PartialEq)]
pub struct RenderProjection {
    pub projection_matrix: Matrix4x4,
}

impl Unique for RenderProjection {
    type Tracking = Untracked;
}

// The active camera's view matrix
// This is a resource since there can only be one camera rendered to a screen.
#[derive(Copy, Clone, PartialEq)]
pub struct RenderView {
    pub view_matrix: Matrix4x4,
}

impl Unique for RenderView {
    type Tracking = Untracked;
}
