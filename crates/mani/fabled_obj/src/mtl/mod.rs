mod loader;
pub use loader::*;

pub const UNKNOWN_PARAM_SUPPORT: [&str; 3] = ["disp", "decal", "refl"];
pub const UNKOWN_PARAM_PBR_SUPPORT: [&str; 9] = [
    "map_Pr", "map_Pm", "map_Ps", "map_Ke", "Pc", "Pcr", "aniso", "anisor", "norm",
];
