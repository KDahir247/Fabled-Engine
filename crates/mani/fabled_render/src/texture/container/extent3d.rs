#[repr(C, align(16))]
#[derive(Default, Debug)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}
