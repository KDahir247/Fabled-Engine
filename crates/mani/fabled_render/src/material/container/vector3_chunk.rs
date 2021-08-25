#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3UIntChunkArray(Vec<[u32; 3]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3SIntChunkArray(Vec<[i32; 3]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3FloatChunkArray(Vec<[f32; 3]>);
