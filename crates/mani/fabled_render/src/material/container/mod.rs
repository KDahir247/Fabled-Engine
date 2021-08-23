mod attribute;
mod key;
mod material;
mod node;
mod target;
mod target_format;
mod tree;

pub use attribute::*;
pub use key::*;
pub use material::*;
pub use node::*;
pub use target::*;
pub use target_format::*;
pub use tree::*;

#[cfg(test)]
mod data_size_test {
    use crate::material::{Attributes, MaterialKey, MaterialTarget, MaterialTree};

    #[test]
    fn test_size() {
        let material_tree_size = std::mem::size_of::<MaterialTree>();
        println!("{:?}", material_tree_size);

        let attributes_size = std::mem::size_of::<Attributes>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let material_key_size = std::mem::size_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::size_of::<MaterialTarget>();
        println!("{}", material_target_size);
    }
}
