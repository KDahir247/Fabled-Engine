mod material_attributes;
mod material_branch;
mod material_key;
mod material_node;
mod material_target;
mod material_tree;
mod material_ty;

pub use material_attributes::*;
pub use material_branch::*;
pub use material_key::*;
pub use material_node::*;
pub use material_target::*;
pub use material_tree::*;
pub use material_ty::*;

#[cfg(test)]
mod data_size_test {
    use crate::material::{
        MaterialAttributes, MaterialBranch, MaterialKey, MaterialNode, MaterialTarget,
        MaterialTree, MaterialValueType,
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

        let value_ty_size = std::mem::size_of::<MaterialValueType>();
        assert_eq!(value_ty_size & (value_ty_size - 1), 0);

        let material_target_size = std::mem::size_of::<MaterialTarget>();
        println!("material target {}", material_target_size);

        let material_node_size = std::mem::size_of::<MaterialNode>();
        assert_eq!(material_node_size & (material_node_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let material_tree_size = std::mem::align_of::<MaterialTree>();
        assert_eq!(material_tree_size & (material_tree_size - 1), 0);

        let material_branch_size = std::mem::align_of::<MaterialBranch>();
        assert_eq!(material_branch_size & (material_branch_size - 1), 0);

        let attributes_size = std::mem::align_of::<MaterialAttributes>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let value_ty = std::mem::align_of::<MaterialValueType>();
        assert_eq!(value_ty & (value_ty - 1), 0);

        let material_key_size = std::mem::align_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::align_of::<MaterialTarget>();
        assert_eq!(material_target_size & (material_target_size - 1), 0);

        let material_node_size = std::mem::align_of::<MaterialNode>();
        assert_eq!(material_node_size & (material_node_size - 1), 0);
    }
}
