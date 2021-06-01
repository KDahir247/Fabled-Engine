pub struct Texture{
    texture : wgpu::Texture,
    view : wgpu::TextureView,
    sampler : wgpu::Sampler
}



impl Texture{
    
    pub fn create_depth_texture(size : &winit::dpi::PhysicalSize<u32> ,device : &wgpu::Device) -> Self{
        
        let extend3d = wgpu::Extent3d{
            width: size.width,
            height: size.width,
            depth_or_array_layers: 0
        };
        
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor{
            label: Some("Depth Texture"),
            size: extend3d,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED
        });


        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_sampler = device.create_sampler(&wgpu::SamplerDescriptor{
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        
        
        todo!()
    }
    

}