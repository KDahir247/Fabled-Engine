#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum TextureSampleType {
    Float { filterable: bool },
    Depth,
    Sint,
    Uint,
}
