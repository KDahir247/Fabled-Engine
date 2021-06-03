use super::{Vertex, Binding, texture, camera};
use wgpu::{VertexBufferLayout, BindGroupLayout};
use anyhow::Context;
use rayon::prelude::*;
use wgpu::util::DeviceExt;
use tobj::LoadError;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex{
    pub position : [f32; 3],
    pub tex_coord : [f32;2],
    pub normal : [f32; 3]
}


impl Vertex for ModelVertex{
    fn desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &
                [
                    wgpu::VertexAttribute{
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0
                    },
                    wgpu::VertexAttribute{
                        format: wgpu::VertexFormat::Float32x2,
                        offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                        shader_location: 1
                    },
                    wgpu::VertexAttribute{
                        format: wgpu::VertexFormat::Float32x3,
                        offset: std::mem::size_of::<[f32;5]>() as wgpu::BufferAddress,
                        shader_location: 2
                    }
                ]
        }
    }
}

pub struct Material{
    name : String,
    bind_group : wgpu::BindGroup,
    texture : texture::Texture
}

impl Material {
    fn new(device : &wgpu::Device,
           name : String,
           texture : texture::Texture,
           layout : &wgpu::BindGroupLayout) -> Self{

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("Material Bind Group"),
            layout,
            entries: &
                [
                    wgpu::BindGroupEntry{
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view)
                    },
                    wgpu::BindGroupEntry{
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler)
                    }
                ]
        });

        Self{
            name,
            bind_group,
            texture
        }

    }

    pub fn create_tex_layout(device : &wgpu::Device) -> wgpu::BindGroupLayout{
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("Diffuse Group Layout"),
            entries: &
                [
                    wgpu::BindGroupLayoutEntry{
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false
                        },
                        count: None
                    },
                    wgpu::BindGroupLayoutEntry{
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { filtering: true, comparison: false },
                        count: None
                    }
                ]
        })
    }
}


struct Mesh{
    name : String,
    vertex_buffer : wgpu::Buffer,
    index_buffer : wgpu::Buffer,
    indices : u32,
    material_id : usize

}

pub struct Model{
    meshes : Vec<Mesh>,
    materials : Vec<Material>
}

//Public
impl Model{

    pub fn load<P : AsRef<std::path::Path>>( //trait wrapper to get the path::Path as ref (as_ref(&self)).
        device : &wgpu::Device,
        queue : &wgpu::Queue,
        path : P,
        tex_layout : &wgpu::BindGroupLayout
    ) -> anyhow::Result<Self> {

        let parent_directory = path.as_ref().parent().unwrap();

        let file_ext  = path.as_ref()
            .extension()
            .context("Failed to identify the extension of the file")?
            .to_str()
            .context("Failed to cast ref OS Str (Reference Operating System String) to a ref Str (ref String Slice)")?;

        match file_ext {
            "obj" => { println!("Reading wavefront obj file at : {}", parent_directory.display());}
             ext => {
                 eprintln!("Unsupported file extension for reading : {}. \nOnly file obj extensions are supported \n", ext)
             }
        }


        let (obj_model, obj_material) = tobj::load_obj(path.as_ref(), &tobj::LoadOptions{
            single_index:  true,
            triangulate:   true,
            ignore_points: true,
            ignore_lines:  true
        }).context("Invalid extension")?;


        match obj_material {
            Ok(_) => {}
            Err(err) => { eprintln!("error : {}", err);}
        }


        //material


        let materials :Vec<Material>  = obj_material?.par_iter().map(|mat : &tobj::Material|{
            let diffuse_path : &String = &mat.diffuse_texture;


            let diffuse_texture = texture::Texture::load(device,queue,parent_directory.join(diffuse_path)).unwrap();

            Material::new(device,
                mat.to_owned().name,
                diffuse_texture,
                tex_layout
            )
        }).collect::<Vec<_>>();



        //mesh
        let meshes : Vec<Mesh> = obj_model.par_iter().map(|m : &tobj::Model|{

           let vertices: Vec<ModelVertex> = (0..m.mesh.positions.len() /3).into_par_iter().map(|i|{
                ModelVertex{
                    position:
                    [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2]
                    ],
                    tex_coord:
                    [
                        m.mesh.texcoords[i * 2],
                        m.mesh.texcoords[i * 2 + 1]
                    ],
                    normal:
                    [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2]
                    ]
                }
            }).collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("vertex buffer"),
                contents: &bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("index buffer"),
                contents: &bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsage::INDEX
            });

            Mesh{
                name: m.name.clone(),
                vertex_buffer,
                index_buffer,
                indices: m.mesh.indices.len() as u32,
                material_id: m.mesh.material_id.unwrap_or(0)
            }
        })
            .collect::<Vec<_>>();

        Ok(
            Model{
                meshes,
                materials
            }
        )
    }
}

pub trait DrawModel<'a, 'b>
    where 'b : 'a
{
    fn draw_model(&mut self, model : &'b Model,  uniform : &'b camera::Uniform);
    fn draw_meshes(&mut self, mesh : &'b Mesh, material : &'b Material, uniform : &'b camera::Uniform);
}

impl<'a,'b> DrawModel<'a, 'b> for wgpu::RenderPass<'a>
    where 'b : 'a {

    fn draw_model(&mut self, model: &'b Model, uniform : &'b camera::Uniform) {
        for m in &model.meshes{
            self.draw_meshes(m, &model.materials[m.material_id], uniform);
        }
    }

    fn draw_meshes(&mut self, mesh: &'b Mesh, material: &'b Material, uniform : &'b camera::Uniform) {

        self.set_bind_group(0,&material.bind_group, &[]);
        self.set_bind_group(1, &uniform.group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        self.draw_indexed(0..mesh.indices,0, 0..1);
    }
}