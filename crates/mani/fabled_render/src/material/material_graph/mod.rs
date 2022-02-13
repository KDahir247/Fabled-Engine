mod material_branch;
mod material_key;
mod material_node;
mod material_primitive_ty;
mod material_tree;
mod material_value;
mod material_value_ty;

pub use material_branch::*;
pub use material_key::*;
pub use material_node::*;
pub use material_primitive_ty::*;
pub use material_tree::*;
pub use material_value::*;
pub use material_value_ty::*;

#[cfg(test)]
mod data_size_test {
    use crate::material::{
        MaterialBranch, MaterialKey, MaterialNode, MaterialPrimitiveType, MaterialTree,
        MaterialValue, MaterialValueType,
    };

    #[test]
    fn test_size() {
        let material_tree_size = std::mem::size_of::<MaterialTree>();
        println!("material tree {}", material_tree_size);

        let material_branch_size = std::mem::size_of::<MaterialBranch>();
        assert_eq!(material_branch_size & (material_branch_size - 1), 0);

        let attributes_size = std::mem::size_of::<MaterialPrimitiveType>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let material_key_size = std::mem::size_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let value_ty_size = std::mem::size_of::<MaterialValueType>();
        assert_eq!(value_ty_size & (value_ty_size - 1), 0);

        let material_target_size = std::mem::size_of::<MaterialValue>();
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

        let attributes_size = std::mem::align_of::<MaterialPrimitiveType>();
        assert_eq!(attributes_size & (attributes_size - 1), 0);

        let value_ty = std::mem::align_of::<MaterialValueType>();
        assert_eq!(value_ty & (value_ty - 1), 0);

        let material_key_size = std::mem::align_of::<MaterialKey>();
        assert_eq!(material_key_size & (material_key_size - 1), 0);

        let material_target_size = std::mem::align_of::<MaterialValue>();
        assert_eq!(material_target_size & (material_target_size - 1), 0);

        let material_node_size = std::mem::align_of::<MaterialNode>();
        assert_eq!(material_node_size & (material_node_size - 1), 0);
    }
}
