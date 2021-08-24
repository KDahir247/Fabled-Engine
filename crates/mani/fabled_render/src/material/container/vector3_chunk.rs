use crate::material::{MaterialNode, Pack, PackMaterial};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3UIntChunkArray(Vec<[u32; 3]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3SIntChunkArray(Vec<[i32; 3]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3FloatChunkArray(Vec<[f32; 3]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec3BoolChunkArray(Vec<[bool; 3]>);

/*impl Pack for Vec3UIntChunkArray {}

impl PackMaterial for Vec3UIntChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec3SIntChunkArray {}

impl PackMaterial for Vec3SIntChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec3FloatChunkArray {}

impl PackMaterial for Vec3FloatChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec3BoolChunkArray {}

impl PackMaterial for Vec3BoolChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}*/
