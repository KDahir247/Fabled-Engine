use crate::component::prelude::*;
use crate::util::texture::*;
use rayon::prelude::*;

use anyhow::Error;
use shipyard::{IntoFastIter, IntoIter, IntoWithId};
use wgpu::util::DeviceExt;

pub fn create_pipeline_system(
    setup: shipyard::UniqueView<Setup>,
    camera: shipyard::UniqueView<Camera>,
    entities: shipyard::EntitiesViewMut,
    model_data: shipyard::ViewMut<ModelData>,
    light: shipyard::View<LightUniform>,
    mut pipeline: shipyard::ViewMut<ModelRenderDetail>,
) {
    model_data.iter().with_id().for_each(|(entity_id, _)| {
        let material = Material::material_layout(&setup.device);

        let mut bind_group_layout: Vec<&wgpu::BindGroupLayout> =
            vec![&material, &camera.uniform.group_layout];

        light.fast_iter().for_each(|uniform| {
            bind_group_layout.push(&uniform.group_layout);
        });

        let render_pipeline = {
            //Shader Module
            let shader_module = setup
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Core Shader Module"),
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                        "../../../shader/shader.wgsl"
                    ))),
                    flags: wgpu::ShaderFlags::all(),
                });

            //Pipeline Layout
            let pipeline_layout =
                setup
                    .device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Layout"),
                        bind_group_layouts: bind_group_layout.as_slice(),
                        push_constant_ranges: &[],
                    });

            setup
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: "vs_main",
                        buffers: &[VertexRaw::desc()], // static unchangable
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: "fs_main",
                        targets: &[wgpu::ColorTargetState {
                            format: setup.swap_chain_desc.format,
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
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
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
    setup: shipyard::UniqueView<Setup>,
    model_data: shipyard::ViewMut<ModelData>,
    mut model_render_detail: shipyard::ViewMut<ModelRenderDetail>,
) -> anyhow::Result<()> {
    (&model_data, &mut model_render_detail)
        .fast_iter()
        .for_each(|(model, render)| {
            let parent_directory: &std::path::Path = model.path.parent().unwrap();

            let file_ext = model.path.extension().unwrap().to_str().unwrap();


            match file_ext {
                "obj" => { println!("Reading wavefront obj file at : {}", parent_directory.display()); }
                ext => {eprintln!("Unsupported file extension for reading : {}. \nOnly file obj extensions are supported \n", ext) }
            }

            let (obj_model, obj_material) = tobj::load_obj(&model.path, &tobj::LoadOptions{
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true
            }).unwrap();

            match obj_material{
                Ok(_) => {}
                Err(err) => {eprintln!("error : {}", err);}
            }

            let obj_material = obj_material.unwrap();

            let materials : Vec<Material> = obj_material
                .par_iter()
                .map(|mat : &tobj::Material|{
                    let diffuse_path : &String = &mat.diffuse_texture;

                    let diffuse_texture = load(&setup.device, &setup.queue, parent_directory.join(diffuse_path)).unwrap();



                    let diffuse_map = Mapping{ texture: diffuse_texture };

                    let material_color = ColorRaw{
                        ambient_color: glam::Vec3::from(mat.ambient).extend(0.5),
                        diffuse_color: glam::Vec3::from(mat.diffuse).extend(0.9),
                        specular_color: glam::Vec3::from(mat.specular).extend(0.8),
                        factor: glam::const_vec3!([mat.shininess, mat.dissolve, mat.optical_density]),
                        ___padding___: 0
                    };

                    Material::new(&setup.device, mat.to_owned().name, material_color, diffuse_map, &render.material_layout)

                }).collect::<Vec<Material>>();

            let meshes : Vec<Mesh> = obj_model
                .par_iter()
                .map(|m : &tobj::Model|{
                    let vertices : Vec<VertexRaw> = (0..m.mesh.positions.len() / 3)
                        .into_par_iter()
                        .map(|i|{
                            let normal : [f32;3] = if m.mesh.normals.is_empty(){
                                [0.0, 0.0, 0.0]
                            }else{
                                [
                                    m.mesh.normals[i * 3],
                                    m.mesh.normals[i * 3 + 1],
                                    m.mesh.normals[i * 3 + 2],
                                ]
                            };

                            let tex_coord = if m.mesh.texcoords.is_empty(){
                                [0.0,0.0]
                            }else{
                                [
                                    m.mesh.texcoords[i * 2],
                                    m.mesh.texcoords[i * 2 + 1]
                                ]
                            };

                            VertexRaw{
                                position: glam::const_vec3!(
                                    [
                                        m.mesh.positions[i * 3],
                                        m.mesh.positions[i * 3 + 1],
                                        m.mesh.positions[i * 3 + 2],
                                    ]),
                                tex_coord: glam::const_vec2!(tex_coord),
                                normal: glam::const_vec3!(normal)
                            }
                        }).collect::<Vec<VertexRaw>>();


                    let vertex_buffer = setup.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsage::VERTEX
                    });

                    let index_buffer = setup.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&m.mesh.indices),
                        usage: wgpu::BufferUsage::INDEX
                    });

                    Mesh{
                        name: m.name.to_owned(),
                        vertex_buffer,
                        index_buffer,
                        indices: m.mesh.indices.len() as u32,
                        material_id: m.mesh.material_id.unwrap_or(0)
                    }
                }).collect::<Vec<Mesh>>();

            render.model = Some(Model{ meshes, materials });
        });

    Ok(())
}
