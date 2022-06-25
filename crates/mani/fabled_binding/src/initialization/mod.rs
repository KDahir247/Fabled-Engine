mod instance;
mod interop;

pub use instance::*;
pub use interop::*;


#[cfg(test)]
mod data_test {
    use crate::LuaInstance;

    #[test]
    fn data_size_test() {
        let lua_instance_size = std::mem::size_of::<LuaInstance>();

        assert_eq!(lua_instance_size & (lua_instance_size - 1), 0);
    }


    #[test]
    fn data_alignment_test() {
        let lua_instance_alignment = std::mem::align_of::<LuaInstance>();
        assert_eq!(lua_instance_alignment & (lua_instance_alignment - 1), 0);
    }
}
