// cascaded shadow maps give you a higher resolution shadow near the camera, and
// have lower resolution the further the cascades are away from the camera
pub fn cascade_resolution(cascade: u32, resolution: f32) -> f32 {
    resolution * f32::exp2((cascade) as f32)
}
