use super::{Vertex, Binding, texture};
use wgpu::{VertexBufferLayout, BindGroupLayout};
use anyhow::Context;
use rayon::prelude::*;

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

struct Material{
    name : String,
    bind_group : wgpu::BindGroup,
    texture : texture::Texture
}

impl Material {
    fn new(device : &wgpu::Device,
           name : String,
           texture : texture::Texture,
           layout : &wgpu::BindGroupLayout) -> Self{

        println!("{}", name); //todo remove

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
}


struct Mesh{
    name : String,
    vertex_buffer : wgpu::Buffer,
    index_buffer : wgpu::Buffer,
    indices : usize,
    material_id : u32

}

pub struct Model{
    meshes : Vec<Mesh>,
    materials : Vec<Material>
}

//Public
impl Model{
    //todo might move to materials struct
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

    pub fn load<P : AsRef<std::path::Path>>( //trait wrapper to get the path::Path as ref (as_ref(&self)).
        device : &wgpu::Device,
        queue : &wgpu::Queue,
        path : P,
        tex_layout : &wgpu::BindGroupLayout
    ) -> anyhow::Result<()> {

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
            single_index: false,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false
        }).context("Invalid extension")?;


        let materials :Vec<Material>  = obj_material?.par_iter().map(|mat : &tobj::Material|{
            let diffuse_path : &String = &mat.diffuse_texture;

            let diffuse_texture = texture::Texture::load(device,queue,parent_directory.join(diffuse_path)).unwrap();

            Material::new(device,
                mat.to_owned().name,
                diffuse_texture,
                tex_layout
            )
        }).collect::<Vec<_>>();


        Ok(())
    }
}

