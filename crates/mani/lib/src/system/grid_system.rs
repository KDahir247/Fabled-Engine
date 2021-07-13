use crate::component::prelude::*;
use shipyard::{IntoFastIter, IntoWithId};
use wgpu::util::DeviceExt;

pub fn create_grid_pipeline_system(
    render: shipyard::UniqueView<RenderData>,
    camera: shipyard::UniqueView<Camera>,
    entities: shipyard::EntitiesViewMut,
    skybox: shipyard::ViewMut<SkyboxData>,
    mut pipeline: shipyard::ViewMut<SkyBoxRenderDetail>,
) {
    skybox.fast_iter().with_id().for_each(|(entity_id, _)| {
        let _skybox =
            Material::material_layout(&render.core.device, wgpu::TextureViewDimension::D2);
        let render_pipeline = {
            let shader_module =
                render
                    .core
                    .device
                    .create_shader_module(&wgpu::ShaderModuleDescriptor {
                        label: Some("Grid Shader"),
                        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                            "../../../fabled_render/src/shader/shader/wgsl/grid.wgsl"
                        ))),
                        flags: wgpu::ShaderFlags::all(),
                    });

            let pipeline_layout =
                render
                    .core
                    .device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Skybox Layout"),
                        bind_group_layouts: &[&camera.uniform.group_layout],
                        push_constant_ranges: &[],
                    });

            render
                .core
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: "fs_main",
                        targets: &[wgpu::ColorTargetState {
                            format: render.info.swap_chain_desc.format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrite::ALL,
                        }],
                    }),
                    primitive: wgpu::PrimitiveState {
                        front_face: wgpu::FrontFace::Ccw,
                        ..Default::default()
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                })
        };

        entities.add_component(
            entity_id,
            &mut pipeline,
            SkyBoxRenderDetail {
                pipeline: render_pipeline,
            },
        )
    });
}
