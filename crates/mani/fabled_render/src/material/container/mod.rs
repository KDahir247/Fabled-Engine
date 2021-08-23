mod attribute;
mod branch;
mod key;
mod material;
mod node;
mod target;
mod target_format;
mod tree;

pub use attribute::*;
pub use branch::*;
pub use key::*;
pub use material::*;
pub use node::*;
pub use target::*;
pub use target_format::*;
pub use tree::*;

#[cfg(test)]
mod data_size_test {
    use crate::material::{
        Attributes, EmptyNode, EmptyTarget, MaterialBranch, MaterialKey, MaterialNode,
        MaterialTarget, MaterialTargetFormat, MaterialTree,
    };

    #[test]
    fn test_size() {
        let material_tree_size = std::mem::size_of::<MaterialTree>();
        println!("material tree {}", material_tree_size);

        let material_branch_size = std::mem::size_of::<MaterialBranch>();
        println!("material branch {}", material_branch_size);

        let attributes_size = std::mem::size_of::<Attributes>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let material_key_size = std::mem::size_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::size_of::<MaterialTarget>();
        println!("material target {}", material_target_size);

        let empty_target_size = std::mem::size_of::<EmptyTarget>();
        println!("empty target {}", empty_target_size);

        let material_node_size = std::mem::size_of::<MaterialNode>();
        println!("material node {}", material_node_size);

        let empty_node_size = std::mem::size_of::<EmptyNode>();
        println!("empty node {}", empty_node_size);

        let material_target_format_size = std::mem::size_of::<MaterialTargetFormat>();
        println!("material target format {}", material_target_format_size);
    }

    #[test]
    fn data_alignment() {
        let material_tree_size = std::mem::align_of::<MaterialTree>();
        println!("material tree {}", material_tree_size);

        let material_branch_size = std::mem::align_of::<MaterialBranch>();
        println!("material branch {}", material_branch_size);

        let attributes_size = std::mem::align_of::<Attributes>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let material_key_size = std::mem::align_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::align_of::<MaterialTarget>();
        println!("material target {}", material_target_size);

        let empty_target_size = std::mem::align_of::<EmptyTarget>();
        println!("empty target {}", empty_target_size);

        let material_node_size = std::mem::align_of::<MaterialNode>();
        println!("material node {}", material_node_size);

        let empty_node_size = std::mem::align_of::<EmptyNode>();
        println!("empty node {}", empty_node_size);

        let material_target_format_size = std::mem::align_of::<MaterialTargetFormat>();
        println!("material target format {}", material_target_format_size);
    }
}
