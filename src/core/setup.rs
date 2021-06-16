use anyhow::Context;
use lib::component::prelude::*;

// This is where all the static storage is created.
pub async fn run(world: &shipyard::World) -> anyhow::Result<()> {
    superluminal_perf::begin_event("Application_SetUp");

    let window = world.borrow::<shipyard::UniqueView<Window>>().unwrap();

    let size = window.raw.inner_size();

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let surface = unsafe { instance.create_surface(&window.raw) };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        })
        .await
        .context("Failed to create graphic adapter")?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Request Device"),
                features: wgpu::Features::NON_FILL_POLYGON_MODE,
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await?;

    println!(
        "Starting Application\nDetail : {:?}",
        format!(
            "CPU = {:?}, GPU-TYPE = {:?}, BACKEND = {:?},",
            adapter.get_info().name,
            adapter.get_info().device_type,
            adapter.get_info().backend
        )
    );

    let swap_chain_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Immediate,
    };

    let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

    superluminal_perf::end_event();

    setup_depth_texture(world, &device, size)?;
    setup_world_camera(world, &device, size)?;
    setup_input_system(world)?;
    setup_world_builder(world)?;

    world.add_unique(Setup {
        size,
        surface,
        adapter,
        device,
        queue,
        swap_chain_desc,
        swap_chain,
    })?;

    Ok(())
}

fn setup_world_camera(
    world: &shipyard::World,
    device: &wgpu::Device,
    size: winit::dpi::PhysicalSize<u32>,
) -> anyhow::Result<()> {
    let orientation = CameraOrientation {
        forward: glam::Vec3::Z * -1.0,
        right: glam::Vec3::X,
        position: glam::Vec3::new(9.0, 2.0, 6.),
        rotation: glam::Vec3::new(0.0, -90.0, 0.0),
        scale: glam::Vec3::ONE,
    };

    let projection = Projection {
        aspect: size.width as f32 / size.height as f32,
        fovy: 60.0_f32.to_radians(),
        znear: 0.1,
        zfar: 100.0,
    };

    let camera_uniform: CameraUniform = CameraUniform::create(device, &orientation, &projection);

    world.add_unique(Camera {
        orientation,
        projection,
        uniform: camera_uniform,
    })?;

    let (mut entities, camera_controller_storage) = world.borrow::<(
        shipyard::EntitiesViewMut,
        shipyard::ViewMut<CameraController>,
    )>()?;

    let camera_controller = CameraController {
        amount_matrix: glam::Mat3::from_cols(
            glam::Vec3::Z * 3.0,
            glam::Vec3::Z * 3.0,
            glam::Vec3::Z * 3.0,
        ),
        amount_rotation: glam::Vec4::W * 0.5,
        amount_scroll: glam::Vec2::Y * 10.0,
    };

    entities.add_entity(camera_controller_storage, camera_controller);

    Ok(())
}

fn setup_depth_texture(
    world: &shipyard::World,
    device: &wgpu::Device,
    size: winit::dpi::PhysicalSize<u32>,
) -> anyhow::Result<()> {
    superluminal_perf::begin_event("Create_Depth_Texture");

    let depth_texture = lib::util::texture::create_depth_texture(device, size);
    world.add_unique(depth_texture)?;

    superluminal_perf::end_event();
    Ok(())
}

fn setup_input_system(world: &shipyard::World) -> anyhow::Result<()> {
    superluminal_perf::begin_event("Create_Input_System");
    world.add_unique(lib::component::input_component::Input::default())?;
    superluminal_perf::end_event();
    Ok(())
}

fn setup_world_builder(world: &shipyard::World) -> anyhow::Result<()> {
    superluminal_perf::begin_event("Create_ECS_System");

    shipyard::Workload::builder("render_update_system")
        .with_system(&lib::system::time_system::calculate_delta_time_system)
        .with_try_system(&lib::system::render_system::begin_render_pass_system)
        .with_system(&lib::system::camera_system::camera_update_system)
        .add_to_world(world)?;

    shipyard::Workload::builder("render_resize_system")
        .with_system(&lib::system::render_system::render_resize_system)
        .add_to_world(world)?;

    shipyard::Workload::builder("load_model_system")
        .with_system(&lib::system::model_system::create_pipeline_system)
        .with_try_system(&lib::system::model_system::load_model_system)
        .add_to_world(world)?;

    superluminal_perf::end_event();

    //shipyard::Workload::builder("render_input_system").with_system();

    Ok(())
}
