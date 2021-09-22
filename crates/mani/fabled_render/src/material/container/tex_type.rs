use serde::*;

// Can compact the information into a single byte, but should be easy to change
// mtl file, so it not desired, since it will add complexity for user.
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct TextureType {
    // Texture is arrayed. 0 is single , 0< is arrayed.
    pub arrayed: u32,

    pub ty: naga::ImageClass,

    pub dimensions: naga::ImageDimension,
}

impl Default for TextureType {
    fn default() -> Self {
        Self {
            ty: naga::ImageClass::Sampled {
                kind: naga::ScalarKind::Sint,
                multi: false,
            },
            dimensions: naga::ImageDimension::D2,
            arrayed: 0,
        }
    }
}
