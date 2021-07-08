/*//in progress
pub struct WgpuRenderResource {
    pub bind_groups:
        std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::BindGroupLayout>>>, //HashMap BindGroupLayout
    pub bind_layouts:
        std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::BindGroup>>>,
    pub shader_modules:
        std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::ShaderModule>>>,
    pub texture_views:
        std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::TextureView>>>,
    pub samplers: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::Sampler>>>,
    pub buffers: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::Buffer>>>,

    pub pipeline:
        std::sync::Arc<std::sync::RwLock<std::collections::HashMap<i32, wgpu::RenderPipeline>>>,
}
*/
