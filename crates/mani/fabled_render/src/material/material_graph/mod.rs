mod material_attributes;
mod material_branch;
mod material_key;
mod material_node;
mod material_target;
mod material_tree;

pub use material_attributes::*;
pub use material_branch::*;
pub use material_key::*;
pub use material_node::*;
pub use material_target::*;
pub use material_tree::*;

#[cfg(test)]
mod data_size_test {
    use crate::material::{
        EmptyNode, EmptyTarget, MaterialAttributes, MaterialBranch, MaterialKey, MaterialNode,
        MaterialTarget, MaterialTree,
    };

    #[test]
    fn test_size() {
        let material_tree_size = std::mem::size_of::<MaterialTree>();
        println!("material tree {}", material_tree_size);

        let material_branch_size = std::mem::size_of::<MaterialBranch>();
        assert_eq!(material_branch_size & (material_branch_size - 1), 0);

        let attributes_size = std::mem::size_of::<MaterialAttributes>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let material_key_size = std::mem::size_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::size_of::<MaterialTarget>();
        println!("material target {}", material_target_size);

        let empty_target_size = std::mem::size_of::<EmptyTarget>();
        assert_eq!(empty_target_size, 0);

        let material_node_size = std::mem::size_of::<MaterialNode>();
        println!("material node {}", material_node_size);

        let empty_node_size = std::mem::size_of::<EmptyNode>();
        assert_eq!(empty_node_size, 0);
    }

    #[test]
    fn data_alignment() {
        let material_tree_size = std::mem::align_of::<MaterialTree>();
        assert_eq!(material_tree_size & (material_tree_size - 1), 0);

        let material_branch_size = std::mem::align_of::<MaterialBranch>();
        assert_eq!(material_branch_size & (material_branch_size - 1), 0);

        let attributes_size = std::mem::align_of::<MaterialAttributes>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let material_key_size = std::mem::align_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::align_of::<MaterialTarget>();
        assert_eq!(material_target_size & (material_target_size - 1), 0);

        let empty_target_size = std::mem::align_of::<EmptyTarget>();
        assert_eq!(empty_target_size & (empty_target_size - 1), 0);

        let material_node_size = std::mem::align_of::<MaterialNode>();
        assert_eq!(material_node_size & (material_node_size - 1), 0);

        let empty_node_size = std::mem::align_of::<EmptyNode>();
        assert_eq!(empty_node_size & (empty_node_size - 1), 0);
    }
}
