
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProjectionMatrix{
    pub inner : fabled_math::Matrix4x4,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewMatrix{
    pub inner : fabled_math::Matrix4x4,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InverseProjectionMatrix{
    pub inner : fabled_math::Matrix4x4,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InverseViewMatrix{
    pub inner : fabled_math::Matrix4x4,
}