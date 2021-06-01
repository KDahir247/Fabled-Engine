use super::{Vertex};
use wgpu::VertexBufferLayout;

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