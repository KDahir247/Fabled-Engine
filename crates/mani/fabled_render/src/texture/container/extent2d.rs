use crate::texture::container::Extent3d;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Extent2d {
    pub width: u32,
    pub height: u32,
}

impl From<Extent2d> for Extent3d {
    fn from(extend_2d: Extent2d) -> Self {
        Extent3d {
            width: extend_2d.width,
            height: extend_2d.height,
            depth_or_array_layers: 0,
        }
    }
}
