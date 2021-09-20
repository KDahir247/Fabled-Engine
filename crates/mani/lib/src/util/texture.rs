use crate::component::render_component::Texture;
use fabled_render::texture::{
    KTXDescriptor, KtxTextureLoader, KtxTranscodeFlag, KtxTranscodeFormat,
};

// todo clean solution.
pub fn load<P: AsRef<std::path::Path>>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: P,
) -> anyhow::Result<Texture> {
    //-------------------------For Png and JPEG Support-------------------------
    //-------------------------Works-------------------------

    // let dyn_img = if path.as_ref().exists() {
    // match path.as_ref().extension() {
    // None => image::open(std::path::Path::new(constant::invalid_map_path().
    // as_str()))?, Some(_) => image::open(path.as_ref())?, //todo check
    // extension if it is a jpg use rgb. if it is a png use rgba }
    // } else {
    // image::open(std::path::Path::new(constant::invalid_map_path().as_str()))?
    // };
    //
    // -------------------------For DDS Support-------------------------
    // -------------------------Works-------------------------
    //

    //-------------------------For KTX2 Support-------------------------

    from_image(device, queue, path.as_ref().to_str().unwrap().to_string())
}

// todo don't get the size from winit::dpi::PhysicalSize, but an Texture struct.
pub fn create_depth_texture(device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) -> Texture {
    let extend3d = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let depth_texture = create_texture(
        device,
        extend3d,
        wgpu::TextureFormat::Depth24PlusStencil8,
        wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
        Some("Depth Texture"),
        1,
    );

    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: None,
        address_mode_u: wgpu::AddressMode::ClampToBorder,
        address_mode_v: wgpu::AddressMode::ClampToBorder,
        address_mode_w: wgpu::AddressMode::ClampToBorder,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: Some(wgpu::CompareFunction::LessEqual),
        border_color: Some(wgpu::SamplerBorderColor::OpaqueBlack),
        ..Default::default()
    });

    Texture {
        view: depth_view,
        sampler: depth_sampler,
    }
}

fn from_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    _test: String,
) -> anyhow::Result<Texture> {
    // todo Maybe make a abstract library is Sol that will read the extension of the
    // file and will use the proper decode  if it fails to read then it will
    // place a png purple image to signify that the texture has not been loaded and
    // failed to load. todo arguments seem kind of big

    // let dds = DdsTextureLoader::default();
    // let dyn_img = dds
    // .load(
    // _test,
    // &fabled_render::TextureDescriptor {
    // flip_axis: Default::default(),
    // dimensions: Default::default(),
    // format: 18,
    // usage: 6,
    // },
    // )
    // .unwrap();

    let ktx = KtxTextureLoader::from_stream(
        std::fs::File::open(_test.as_str()).unwrap(),
        &KTXDescriptor {
            flip_axis: fabled_render::texture::FlipAxis::FlipY,
            transcode_flag: KtxTranscodeFlag::HIGHEST_QUALITY,
            transcode_format: KtxTranscodeFormat::RGBA32,
        },
    )
    .unwrap();

    let extend_test = wgpu::Extent3d {
        width: ktx.size.width,
        height: ktx.size.height,
        depth_or_array_layers: ktx.size.depth_or_array_layers,
    };

    extend_test.physical_size(wgpu::TextureFormat::Rgba8UnormSrgb);

    let diffuse_texture = create_texture(
        device,
        extend_test,
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
        &ktx.data,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(core::num::NonZeroU32::new(ktx.rows_per_image).unwrap()),
            rows_per_image: Some(core::num::NonZeroU32::new(ktx.size.height).unwrap()),
        },
        extend_test,
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
