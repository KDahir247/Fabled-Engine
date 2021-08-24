use crate::material::{MaterialNode, Pack, PackMaterial};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec2UIntChunkArray(Vec<[u32; 2]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec2SIntChunkArray(Vec<[i32; 2]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec2FloatChunkArray(Vec<[f32; 2]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec2BoolChunkArray(Vec<[bool; 2]>);

/*impl Pack for Vec2UIntChunkArray {}

impl PackMaterial for Vec2UIntChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec2SIntChunkArray {}

impl PackMaterial for Vec2SIntChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec2FloatChunkArray {}

impl PackMaterial for Vec2FloatChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec2BoolChunkArray {}

impl PackMaterial for Vec2BoolChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}*/
