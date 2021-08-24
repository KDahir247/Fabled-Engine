use crate::material::MaterialNode;

mod vector2_chunk;
mod vector3_chunk;
mod vector4_chunk;

pub trait Pack {
    //todo will have a universal Pack method where the type must be Pod and Zeroable.
}

pub trait PackMaterial: Pack {
    fn pack_material(_: Vec<MaterialNode>);
}

#[cfg(test)]
mod data_test {

    #[test]
    fn data_size() {}

    #[test]
    fn data_alignment() {}
}
