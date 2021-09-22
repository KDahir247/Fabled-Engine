// RMA is usually dealt with PBR (physically base rendering) where the Roughness
// map, Metallic Map, and Ambient Occlusion map is stored in one map. Where the
// Red channel store the Roughness, Green channel stores the Metallic and the
// Blue channel stores the Ambient Occlusion.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SupportRMA {
    True,
    False,
}
