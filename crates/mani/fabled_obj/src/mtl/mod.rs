mod loader;
pub use loader::*;

// Texture String, Texture String, Texture String,
pub const UNKNOWN_PARAM_SUPPORT: [&str; 3] = ["disp", "decal", "refl"];

// roughness map, metallic map, sheen map, clear coat thickness, clear coat
// roughness, emissive texture, anisotropy, anisotropy rotation, normal map
pub const UNKNOWN_PARAM_PBR_SUPPORT: [&str; 9] = [
    "map_Pr", "map_Pm", "map_Ps", "Pc", "Pcr", "map_Ke", "aniso", "anisor", "norm",
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextureTarget {
    Standard,
    PBR,
}

#[cfg(test)]
mod data_test {
    use crate::TextureTarget;

    #[test]
    fn data_size() {
        let texture_target_size = std::mem::size_of::<TextureTarget>();
        assert_eq!(texture_target_size & (texture_target_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let texture_target_alignment = std::mem::align_of::<TextureTarget>();
        assert_eq!(texture_target_alignment & (texture_target_alignment - 1), 0);
    }
}
