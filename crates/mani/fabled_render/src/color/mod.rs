mod container;
mod contract;
mod conversion;
mod gamut;
mod space;
mod util;

pub use container::*;
pub use conversion::*;
use fabled_math::vector_math::component_sum;
use fabled_math::Vector3;
pub use gamut::*;
pub use space::*;
pub use util::*;

pub fn xy_y_to_xyz(xy_y: Vector3) -> Vector3 {
    // Y / y
    let a = xy_y.z() / xy_y.y();
    let b = 1.0 - xy_y.x() - xy_y.y();

    // x * (Y / y)
    let x = xy_y.x() * a;
    // Z = (1-x-y)Y / y
    let z = b * a;
    // Y = Y
    let y = xy_y.z();

    Vector3::set(x, y, z)
}

pub fn xyz_to_xy_y(xyz: Vector3) -> Vector3 {
    // 1.0 / (X + Y + Z)
    let intermediate = 1.0 / (component_sum(xyz.value));

    // x = X / (X + Y + Z)
    let x = xyz.x() * intermediate;
    // y = Y / (X + Y + Z)
    let y = xyz.y() * intermediate;
    // Y = Y
    let _y = xyz.y();

    Vector3::set(x, y, _y)
}
