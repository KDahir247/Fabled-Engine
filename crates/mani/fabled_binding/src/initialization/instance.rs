// This will be added as a unique to the ecs system, since we want this to be
// "static" in a sense where there is only one instance, but we don't want to
// make it global

use crate::{LuaOption, LuaSafetySate, StdLib, System};
use mlua::{FromLuaMulti, ToLuaMulti};
use std::env::Args;
use std::fmt::Debug;
use std::future::Future;

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
        A: 'static + mlua::FromLuaMulti<'lua>,
        R: 'static + mlua::ToLuaMulti<'lua>,
        T: 'static + System<'static, A, R> + Fn<A, Output = R>,
    >(
        &'lua self,
        function: T,
        lua_fn_name: &str,
    ) -> mlua::Result<()> {
        let lua_func = self
            .context
            .create_function(move |x, parameter: A| Ok(function.call(parameter)))?;

        self.context.globals().raw_set(lua_fn_name, lua_func)?;

        Ok(())
    }

    pub fn create_mut_function<
        A: 'static + mlua::FromLuaMulti<'lua>,
        R: 'static + mlua::ToLuaMulti<'lua>,
        T: 'static + System<'static, A, R> + FnMut<A, Output = R>,
    >(
        &'lua self,
        mut function: T,
        lua_fn_name: &str,
    ) -> mlua::Result<()> {
        let mut_lua_fn = self
            .context
            .create_function_mut(move |x, parameter: A| Ok(function.call_mut(parameter)))?;


        self.context.globals().raw_set(lua_fn_name, mut_lua_fn)?;


        Ok(())
    }


    // pub fn create_async_function<
    //     A: 'static + mlua::FromLuaMulti<'lua>,
    //     R: 'static + mlua::ToLuaMulti<'lua>,
    //     T: 'static + System<'static, A, R> + Fn<A, Output = impl Future<Output =
    // R>>, >(
    //     &'lua self,
    //     function: T,
    //     lua_fn_name: &str,
    // ) -> mlua::Result<()> {
    //     let async_lua_fn = self
    //         .context
    //         .create_async_function(move |x, parameter: A|
    // Ok(function.call(parameter)))?;
    //
    //     self.context.globals().raw_set(lua_fn_name, async_lua_fn)?;
    //
    //
    //     Ok(())
    // }

    // create async fn
}


#[cfg(test)]
mod test {
    use crate::{LuaInstance, System};
    use std::sync::Arc;

    fn times_two(x: usize) -> usize {
        x * 2
    }

    fn add_two(x: usize) -> usize {
        x + 2
    }

    fn power_of(x: f32, y: f32) -> f32 {
        x.powf(y)
    }

    fn modify_x(mut y: f32) -> f32 {
        y += 2.0;
        y
    }

    #[test]
    fn rust_fn_call_test() {
        let lua = LuaInstance::default();

        lua.create_function(times_two, "times_two").unwrap();
        lua.create_function(add_two, "add_two").unwrap();

        lua.context
            .load(&std::fs::read_to_string("./lua_src/fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }

    #[test]
    fn rust_multi_parameter_fn_test() {
        let lua = LuaInstance::default();

        lua.create_function(power_of, "power_of").unwrap();

        lua.context
            .load(&std::fs::read_to_string("./lua_src/multi_param_fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }


    #[test]
    fn rust_mut_fn_test() {
        let lua = LuaInstance::default();

        lua.create_mut_function(modify_x, "modify_x").unwrap();

        lua.context
            .load(&std::fs::read_to_string("./lua_src/mut_fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }


    #[test]
    fn rust_mut_fn_call_test() {
        let lua = LuaInstance::default();
    }
}
