use crate::texture::texture_sampler::TextureSampler;

#[repr(align(128))]
pub struct Texture {
    pub data: Vec<u8>,
    pub size: wgpu::Extent3d,
    pub format: wgpu::TextureFormat,
    pub sampler: TextureSampler,
    pub dimensions: wgpu::TextureViewDimension,
}
