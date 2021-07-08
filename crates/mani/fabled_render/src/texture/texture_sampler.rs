use wgpu::SamplerDescriptor;
//todo currently 44 either decrease or increase
#[repr(align(64))]
pub struct TextureSampler {
    pub u_edge: wgpu::AddressMode,
    pub v_edge: wgpu::AddressMode,
    pub w_edge: wgpu::AddressMode,
    pub mag_filter: wgpu::FilterMode,
    pub min_filter: wgpu::FilterMode,
    pub mip_map_filter: wgpu::FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<wgpu::CompareFunction>,
    pub anisotropy_clamp: Option<std::num::NonZeroU8>,
    pub border_color: Option<wgpu::SamplerBorderColor>,
}

impl Default for TextureSampler {
    fn default() -> Self {
        Self {
            u_edge: wgpu::AddressMode::ClampToEdge,
            v_edge: wgpu::AddressMode::ClampToEdge,
            w_edge: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mip_map_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: Some(wgpu::CompareFunction::LessEqual),
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

impl From<wgpu::SamplerDescriptor<'static>> for TextureSampler {
    fn from(sampler_descriptor: SamplerDescriptor<'static>) -> Self {
        Self {
            u_edge: sampler_descriptor.address_mode_u,
            v_edge: sampler_descriptor.address_mode_v,
            w_edge: sampler_descriptor.address_mode_w,
            mag_filter: sampler_descriptor.mag_filter,
            min_filter: sampler_descriptor.min_filter,
            mip_map_filter: sampler_descriptor.mipmap_filter,
            lod_min_clamp: sampler_descriptor.lod_min_clamp,
            lod_max_clamp: sampler_descriptor.lod_max_clamp,
            compare: sampler_descriptor.compare,
            anisotropy_clamp: sampler_descriptor.anisotropy_clamp,
            border_color: sampler_descriptor.border_color,
        }
    }
}
