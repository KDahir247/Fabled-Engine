// This will be added as a unique to the ecs system, since we want this to be
// "static" in a sense where there is only one instance, but we don't want to
// make it global

use crate::{LuaOption, LuaSafetySate, StdLib};
use mlua::FromLuaMulti;
use std::fmt::Debug;

pub struct LuaInstance {
    context: mlua::Lua,
    safety_state: LuaSafetySate,
}

impl Default for LuaInstance {
    fn default() -> Self {
        LuaInstance::new(StdLib::ALL, LuaOption::new())
    }
}

impl LuaInstance {
    pub unsafe fn new_unsafe(lua_lib: StdLib, lua_option: LuaOption) -> Self {
        let lua = mlua::Lua::unsafe_new_with(lua_lib.into(), lua_option.into());
        Self {
            context: lua,
            safety_state: LuaSafetySate::Unsafe,
        }
    }

    pub fn new(lua_lib: StdLib, lua_option: LuaOption) -> Self {
        // todo issue with lua_lib and lua_option cause ffi error
        // let lua =
        //    mlua::Lua::new_with(lua_lib.into(),
        // lua_option.into()).unwrap_or(mlua::Lua::new());

        let lua = mlua::Lua::new();

        Self {
            context: lua,
            safety_state: LuaSafetySate::Safe,
        }
    }
}

impl<'lua> LuaInstance {
    pub fn create_function<
        A: 'static + mlua::FromLua<'lua> + mlua::FromLuaMulti<'lua>,
        R: mlua::ToLua<'lua> + mlua::FromLuaMulti<'lua>,
        T: 'static + Fn(A) -> R,
    >(
        &'lua self,
        function: T,
        lua_fn_name: &str,
    ) -> mlua::Result<()> {
        let lua_func = self
            .context
            .create_function(move |x, parameter: A| Ok(function.call((parameter,))))?;

        self.context.globals().raw_set(lua_fn_name, lua_func)?;

        Ok(())
    }

    // create mut fn


    // create async fn


    // create async mut fn?
}


impl LuaInstance {}

#[cfg(test)]
mod test {
    use crate::LuaInstance;

    fn times_two(x: usize) -> usize {
        x * 2
    }

    fn add_two(x: usize) -> usize {
        x + 2
    }

    #[test]
    fn create_function_test() {
        let lua = LuaInstance::default();

        lua.create_function(times_two, "times_two").unwrap();
        lua.create_function(add_two, "add_two").unwrap();

        lua.context
            .load(&std::fs::read_to_string("./lua_src/rust_fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }
}
