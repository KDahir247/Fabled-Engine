#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BiTangent {
    x: [f32; 16],
    y: [f32; 16],
    z: [f32; 16],
    w: [f32; 16], // handedness
}
