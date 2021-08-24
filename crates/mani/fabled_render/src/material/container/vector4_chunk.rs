use crate::material::{MaterialNode, Pack, PackMaterial};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4UIntChunkArray(Vec<[u32; 4]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4SIntChunkArray(Vec<[i32; 4]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4FloatChunkArray(Vec<[f32; 4]>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vec4BoolChunkArray(Vec<[bool; 4]>);

/*impl Pack for Vec4UIntChunkArray {}

impl PackMaterial for Vec4UIntChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec4SIntChunkArray {}

impl PackMaterial for Vec4SIntChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec4FloatChunkArray {}

impl PackMaterial for Vec4FloatChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}

impl Pack for Vec4BoolChunkArray {}

impl PackMaterial for Vec4BoolChunkArray {
    fn pack_material(_: Vec<MaterialNode>) {}
}
*/
