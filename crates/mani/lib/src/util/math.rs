// Projection
// a . b * b / b^2
// or a . b * b / 1.0 (normalized)
pub fn projection(axis: glam::Vec3, direction: glam::Vec3) -> glam::Vec3 {
    let axis = axis.normalize();
    direction.dot(axis) * axis
}

// Rejection
// a - (a . b * b / b^2)
// or a - a . b * b / 1 (normalized)
pub fn reject(axis: glam::Vec3, direction: glam::Vec3) -> glam::Vec3 {
    let axis = axis.normalize();
    direction - axis * direction.dot(axis)
}
