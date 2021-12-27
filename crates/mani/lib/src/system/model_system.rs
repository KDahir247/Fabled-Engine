use crate::component::prelude::*;
use crate::util::texture::*;
use rayon::prelude::*;

use shipyard::{IntoFastIter, IntoWithId};
use wgpu::util::DeviceExt;

pub fn create_model_pipeline_system(
    render: shipyard::UniqueView<RenderData>,
    camera: shipyard::UniqueView<Camera>,
    entities: shipyard::EntitiesViewMut,
    model_data: shipyard::ViewMut<ModelData>,
    light: shipyard::View<LightUniform>,
    mut pipeline: shipyard::ViewMut<ModelRenderDetail>,
) {
    model_data.fast_iter().with_id().for_each(|(entity_id, _)| {
        let material =
            Material::material_layout(&render.core.device, wgpu::TextureViewDimension::D2);

        let mut bind_group_layout: Vec<&wgpu::BindGroupLayout> =
            vec![&material, &camera.uniform.group_layout];

        light.fast_iter().for_each(|uniform| {
            bind_group_layout.push(&uniform.group_layout);
        });

        let render_pipeline = {
            // Shader Module
            let shader_module =
                render
                    .core
                    .device
                    .create_shader_module(&wgpu::ShaderModuleDescriptor {
                        label: Some("Core Shader Module"),
                        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                            "../../../fabled_render/src/shader/shader/wgsl/standard.wgsl"
                        ))),
                        flags: wgpu::ShaderFlags::all(),
                    });

            // Pipeline Layout
            let pipeline_layout =
                render
                    .core
                    .device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Layout"),
                        bind_group_layouts: bind_group_layout.as_slice(),
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
                        buffers: &[VertexRaw::desc()],
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
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        clamp_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
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
            ModelRenderDetail {
                pipeline: render_pipeline,
                material_layout: material,
                model: None,
            },
        );
    });
}

pub fn load_model_system(
    render: shipyard::UniqueView<RenderData>,
    model_data: shipyard::ViewMut<ModelData>,
    mut model_render_detail: shipyard::ViewMut<ModelRenderDetail>,
) -> anyhow::Result<()> {
    (&model_data, &mut model_render_detail)
        .fast_iter()
        .for_each(|(model, render_detail)| {
            let parent_directory: &std::path::Path = model.path.parent().unwrap();

            let obj_loader = fabled_obj::ObjLoader::default();


            let (model, materials) = obj_loader.load(&model.path, 4).unwrap();

            let materials: Vec<Material> = materials
                .par_iter()
                .map(|mat| {
                    let diffuse_path: &String = &mat.diffuse_texture;


                    // todo once loaded we dont need to store it????????????
                    let diffuse_texture = load(
                        &render.core.device,
                        &render.pass.queue,
                        parent_directory.join(diffuse_path),
                    )
                    .unwrap();

                    let diffuse_map = Mapping {
                        texture: diffuse_texture,
                    };

                    let material_color = ColorRaw {
                        ambient_color: glam::Vec3::from(mat.ambient).extend(0.5),
                        diffuse_color: glam::Vec3::from(mat.diffuse).extend(0.9),
                        specular_color: glam::Vec3::from(mat.specular).extend(0.8),
                        factor: glam::const_vec3!([
                            mat.shininess,
                            mat.dissolve,
                            mat.optical_density
                        ]),
                        ___padding___: 0,
                    };

                    Material::new(
                        &render.core.device,
                        mat.to_owned().name,
                        material_color,
                        diffuse_map,
                        &render_detail.material_layout,
                    )
                })
                .collect::<Vec<Material>>();


            let meshes: Vec<Mesh> =
                model
                    .meshes
                    .par_iter()
                    .map(|m| {
                        let mesh = m.to_owned();
                        let vertices = mesh.vertices;

                        let mut ab = Vec::new();

                        if let fabled_render::mesh::Indices::U32(a) = mesh.indices {
                            ab = a;
                        }


                        let vertex_buffer = render.core.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(&vertices),
                                usage: wgpu::BufferUsage::VERTEX,
                            },
                        );

                        let index_buffer = render.core.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Index Buffer"),
                                contents: bytemuck::cast_slice(ab.as_slice()),
                                usage: wgpu::BufferUsage::INDEX,
                            },
                        );

                        // todo remove.
                        Mesh {
                            vertex_buffer,
                            index_buffer,
                            indices: ab.len() as u32,
                            material_id: m.material_id as usize,
                        }
                    })
                    .collect::<Vec<Mesh>>();

            render_detail.model = Some(Model { meshes, materials });
        });

    Ok(())
}
