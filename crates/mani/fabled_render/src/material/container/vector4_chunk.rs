#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4UIntChunkArray(Vec<[u32; 4]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4SIntChunkArray(Vec<[i32; 4]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4FloatChunkArray(Vec<[f32; 4]>);
