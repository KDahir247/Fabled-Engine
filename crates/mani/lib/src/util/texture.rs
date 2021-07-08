use super::constant;
use crate::component::render_component::Texture;

pub fn load<P: AsRef<std::path::Path>>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: P,
) -> anyhow::Result<Texture> {
    let dyn_img = if path.as_ref().exists() {
        match path.as_ref().extension() {
            None => image::open(std::path::Path::new(constant::invalid_map_path().as_str()))?,
            Some(_) => image::open(path.as_ref())?, //todo check extension if it is a jpg use rgb. if it is a png use rgba
        }
    } else {
        image::open(std::path::Path::new(constant::invalid_map_path().as_str()))?
    };

    from_image(device, queue, dyn_img)
}

pub fn create_depth_texture(device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) -> Texture {
    let extend3d = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let depth_texture = create_texture(
        device,
        extend3d,
        wgpu::TextureFormat::Depth32Float,
        wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        Some("Depth Texture"),
        1,
    );

    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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

    Texture {
        view: depth_view,
        sampler: depth_sampler,
    }
}

#[allow(dead_code)] //TODO note Dead Code
pub fn from_bytes(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    img_buffer: &[u8],
) -> anyhow::Result<Texture> {
    let dyn_img = image::load_from_memory(img_buffer)?;

    from_image(device, queue, dyn_img)
}

fn from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    dyn_img: image::DynamicImage,
) -> anyhow::Result<Texture> {
    let reversed_rgba8_img = dyn_img.to_rgba8();

    let dimensions = reversed_rgba8_img.dimensions();

    let rgba8_img = reversed_rgba8_img
        .chunks(dimensions.0 as usize * 4)
        .rev()
        .flat_map(|row| row.iter())
        .cloned()
        .collect::<Vec<_>>();

    let extend3d = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    }
    .physical_size(wgpu::TextureFormat::Rgba8UnormSrgb);

    let diffuse_texture = create_texture(
        device,
        extend3d,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED,
        Some("Diffuse Texture"),
        1,
    );

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &rgba8_img,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(core::num::NonZeroU32::new(4 * dimensions.0).unwrap()),
            rows_per_image: Some(core::num::NonZeroU32::new(dimensions.1).unwrap()),
        },
        extend3d,
    );

    let diffuse_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Diffuse Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        anisotropy_clamp: None,
        ..Default::default()
    });

    Ok(Texture {
        view: diffuse_view,
        sampler: diffuse_sampler,
    })
}

fn create_texture(
    device: &wgpu::Device,
    size: wgpu::Extent3d,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsage,
    label: Option<&str>,
    mip_map_level: u8,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: mip_map_level as u32,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
    })
}
