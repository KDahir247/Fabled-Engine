mod byte_ty;
mod func_ty;
mod gc_gen;
mod option;
mod std_lib;

pub use byte_ty::*;
pub use func_ty::*;
pub use gc_gen::*;
pub use option::*;
pub use std_lib::*;


#[cfg(test)]
mod data_test {
    use crate::{FunctionType, GCMultiplier, LuaOption};

    #[test]
    fn data_size() {
        let function_type_size = std::mem::size_of::<FunctionType>();
        assert_eq!(function_type_size & (function_type_size - 1), 0);

        let gc_multiplier_size = std::mem::size_of::<GCMultiplier<0, 10>>();
        assert_eq!(gc_multiplier_size & (gc_multiplier_size - 1), 0);

        let lua_option_size = std::mem::size_of::<LuaOption>();
        assert_eq!(lua_option_size & (lua_option_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let function_type_alignment = std::mem::align_of::<FunctionType>();
        assert_eq!(function_type_alignment & (function_type_alignment - 1), 0);

        let gc_multiplier_alignment = std::mem::align_of::<GCMultiplier<0, 10>>();
        assert_eq!(gc_multiplier_alignment & (gc_multiplier_alignment - 1), 0);

        let lua_option_alignment = std::mem::align_of::<LuaOption>();
        assert_eq!(lua_option_alignment & (lua_option_alignment - 1), 0);
    }
}
