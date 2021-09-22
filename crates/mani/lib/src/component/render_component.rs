// todo maybe remove both
pub struct Texture {
    pub view: wgpu::TextureView, // 24 bytes todo maybe remove this.
    pub sampler: wgpu::Sampler,  // 16 bytes
}

pub struct Core {
    pub device: wgpu::Device,
}

pub struct Pass {
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
}

#[repr(align(32))]
pub struct Info {
    pub swap_chain_desc: wgpu::SwapChainDescriptor,
}

pub struct Setup {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
}

pub struct RenderData {
    pub core: Core,
    pub pass: Pass,
    pub info: Info,
    pub setup: Setup,
}

pub struct RenderPipeline {}
