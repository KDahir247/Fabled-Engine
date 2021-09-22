use crate::texture::container::BorderColor;

// Filter and Sampler
#[repr(align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureSampler {
    pub mip_map_filter: i32,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub anisotropy_clamp: Option<std::num::NonZeroU8>,
    pub compare: Option<u8>,
    pub u_edge: u8,
    pub v_edge: u8,
    pub w_edge: u8,
    pub mag_filter: u8,
    pub min_filter: u8,
    pub border_color: Option<BorderColor>,
}

impl Default for TextureSampler {
    fn default() -> Self {
        Self {
            u_edge: 0,
            v_edge: 0,
            w_edge: 0,
            mag_filter: 0,
            min_filter: 1,
            mip_map_filter: 1,
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: Some(4),
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}
