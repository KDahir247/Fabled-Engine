use crate::component::render_component::Texture;
use wgpu::util::DeviceExt;

pub struct Material {
    pub mat_name: String,           // 24 bytes
    pub mat_color: ColorRaw,        // 64 bytes
    pub mat_mapping: Mapping,       // 40 bytes
    pub mat_group: wgpu::BindGroup, // 16 bytes
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
            contents: bytemuck::cast_slice(&[
                material_color.ambient_color,
                material_color.diffuse_color,
                material_color.specular_color,
                material_color.factor.extend(1.0),
            ]),
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
            mat_mapping: material_mapping,
            mat_group,
        }
    }

    pub fn material_layout(
        device: &wgpu::Device,
        view_dimension: wgpu::TextureViewDimension,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Material Layout"),
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
                        view_dimension,
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
    // RGB-Scalar
    pub ambient_color: glam::Vec4,
    pub diffuse_color: glam::Vec4,
    pub specular_color: glam::Vec4,
    // Shininess, Dissolve, Optical Density
    pub factor: glam::Vec3,
    pub ___padding___: u32,
}

pub struct Mapping {
    pub texture: Texture,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub position: glam::Vec3,
    pub tex_coord: glam::Vec2,
    pub normal: glam::Vec3,
    pub tangent: glam::Vec4,
    pub bi_tangent: glam::Vec4, // we don't want the bi_tangent in the shader
}

impl VertexRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexRaw>() as wgpu::BufferAddress,
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
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
            ],
        }
    }
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub indices: u32,
    pub material_id: usize,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct ModelData {
    pub path: std::path::PathBuf,
    pub shader_path: String,
}

pub struct ModelRenderDetail {
    pub model: Option<Model>,
    pub pipeline: wgpu::RenderPipeline,
    pub material_layout: wgpu::BindGroupLayout,
}
