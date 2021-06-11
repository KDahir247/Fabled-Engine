use super::{camera, light, texture};

use anyhow::Context;
use rayon::prelude::*;
use wgpu::util::DeviceExt;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub position: glam::Vec3,
    pub tex_coord: glam::Vec2,
    pub normal: glam::Vec3,
}

impl Vertex for VertexRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
            ],
        }
    }
}

pub struct Material {
    mat_name: String,
    mat_color: ColorRaw,
    pub mat_group: wgpu::BindGroup,
    mat_mapping: Mapping, //todo doesn't support other mapping type such as normal map, specular map. Just diffuse map
}

impl Material {
    pub fn new(
        device: &wgpu::Device,
        material_name: String,
        material_color: ColorRaw,
        material_mapping: Mapping,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Color Buffer"),
            contents: &bytemuck::cast_slice(&[material_color]),
            usage: wgpu::BufferUsage::UNIFORM,
        });

        let mat_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: color_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&material_mapping.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&material_mapping.texture.sampler),
                },
            ],
        });

        Self {
            mat_name: material_name,
            mat_color: material_color,
            mat_group,
            mat_mapping: material_mapping,
        }
    }

    pub fn create_material_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Material Uniform Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: true,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorRaw {
    //RGB
    pub ambient_color: glam::Vec4,
    pub diffuse_color: glam::Vec4,
    pub specular_color: glam::Vec4,
    //Shininess, Dissolve.
    pub factor: glam::Vec3,
    _padding: u32,
}

impl ColorRaw {
    pub fn new(
        ambient_color: glam::Vec4,
        diffuse_color: glam::Vec4,
        specular_color: glam::Vec4,
        shininess: f32,
        dissolve: f32,
        optical_density: f32,
    ) -> Self {
        Self {
            ambient_color,
            diffuse_color,
            specular_color,
            factor: glam::const_vec3!([shininess, dissolve, optical_density]),
            _padding: 0,
        }
    }
}

pub struct Mapping {
    pub texture: texture::Texture, //there will be multiple mapping (diffuse, normal, etc...)
}

pub struct Mesh {
    name: String,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices: u32,
    material_id: usize,
}

pub struct Model {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
}

//Public
impl Model {
    pub fn load<P: AsRef<std::path::Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: P,
        material_layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<Self> {
        let parent_directory = path.as_ref().parent().unwrap();

        let file_ext  = path.as_ref()
            .extension()
            .context("Failed to identify the extension of the file")?
            .to_str()
            .context("Failed to cast ref OS Str (Reference Operating System String) to a ref Str (ref String Slice)")?;

        match file_ext {
            "obj" => {
                println!(
                    "Reading wavefront obj file at : {}",
                    parent_directory.display()
                );
            }
            ext => {
                eprintln!("Unsupported file extension for reading : {}. \nOnly file obj extensions are supported \n", ext)
            }
        }

        let (obj_model, obj_material) = tobj::load_obj(
            path.as_ref(),
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
            },
        )
        .context("Invalid extension")?;

        match obj_material {
            Ok(_) => {}
            Err(err) => {
                eprintln!("error : {}", err);
            }
        }

        //material
        let materials: Vec<Material> = obj_material?
            .par_iter()
            .map(|mat: &tobj::Material| {
                let diffuse_path: &String = &mat.diffuse_texture;

                let diffuse_texture =
                    texture::Texture::load(device, queue, parent_directory.join(diffuse_path))
                        .unwrap();

                //
                let diffuse_map = Mapping {
                    texture: diffuse_texture,
                };

                let material_color = ColorRaw::new(
                    glam::Vec3::from(mat.ambient).extend(0.5),
                    glam::Vec3::from(mat.diffuse).extend(0.9),
                    glam::Vec3::from(mat.specular).extend(0.8),
                    mat.shininess,
                    mat.dissolve,
                    mat.optical_density,
                );

                Material::new(
                    device,
                    mat.to_owned().name,
                    material_color,
                    diffuse_map,
                    material_layout,
                )
            })
            .collect::<Vec<Material>>();

        //mesh
        let meshes: Vec<Mesh> = obj_model
            .par_iter()
            .map(|m: &tobj::Model| {
                let vertices: Vec<VertexRaw> = (0..m.mesh.positions.len() / 3)
                    .into_par_iter()
                    .map(|i| {
                        let normal = if m.mesh.normals.is_empty() {
                            [0.0, 0.0, 0.0]
                        } else {
                            [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ]
                        };

                        let tex_coord = if m.mesh.texcoords.is_empty() {
                            [0.0, 0.0]
                        } else {
                            [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]]
                        };

                        VertexRaw {
                            position: glam::const_vec3!([
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2]
                            ]),
                            tex_coord: glam::const_vec2!(tex_coord),
                            normal: glam::const_vec3!(normal),
                        }
                    })
                    .collect::<Vec<VertexRaw>>();

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("vertex buffer"),
                    contents: &bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsage::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("index buffer"),
                    contents: &bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsage::INDEX,
                });

                Mesh {
                    name: m.name.to_owned(),
                    vertex_buffer,
                    index_buffer,
                    indices: m.mesh.indices.len() as u32,
                    material_id: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<Mesh>>();

        Ok(Model { meshes, materials })
    }
}

pub trait DrawModel<'a, 'b>
where
    'b: 'a,
{
    fn draw_model(
        &mut self,
        model: &'b Model,
        uniform: &'b camera::Uniform,
        light: &'b light::Uniform,
    );
    fn draw_meshes(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        uniform: &'b camera::Uniform,
        light: &'b light::Uniform,
    );
}

impl<'a, 'b> DrawModel<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_model(
        &mut self,
        model: &'b Model,
        uniform: &'b camera::Uniform,
        light: &'b light::Uniform,
    ) {
        for m in &model.meshes {
            self.draw_meshes(m, &model.materials[m.material_id], uniform, light);
        }
    }

    fn draw_meshes(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        uniform: &'b camera::Uniform,
        light: &'b light::Uniform,
    ) {
        self.set_bind_group(0, &material.mat_group, &[]);
        self.set_bind_group(1, &uniform.group, &[]);
        self.set_bind_group(2, &light.group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        self.draw_indexed(0..mesh.indices, 0, 0..1);
    }
}
