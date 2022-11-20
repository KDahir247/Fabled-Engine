pub use aperture::*;
pub use aspect_ratio::*;
pub use clipping_plane::*;
pub use f_stop::*;
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
