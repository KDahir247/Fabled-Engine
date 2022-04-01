// This will be added as a unique to the ecs system, since we want this to be
// "static" in a sense where there is only one instance, but we don't want to
// make it global

use crate::{GenerationalMultiplier, LuaOption, StdLib, System};
use mlua::{FromLuaMulti, Function, ToLuaMulti};
use std::env::Args;
use std::fmt::Debug;
use std::future::Future;

pub struct LuaInstance {
    context: mlua::Lua,
    gc_mode: mlua::GCMode,
}

impl Default for LuaInstance {
    fn default() -> Self {
        LuaInstance::new(StdLib::ALL, LuaOption::new())
    }
}

impl LuaInstance {
    pub unsafe fn new_unsafe(lua_lib: StdLib, lua_option: LuaOption) -> Self {
        let instance = Self {
            context: mlua::Lua::unsafe_new_with(lua_lib.into(), lua_option.into()),
            gc_mode: mlua::GCMode::Generational,
        };

        instance.gc_gen(
            GenerationalMultiplier::new::<20>(),
            GenerationalMultiplier::new::<100>(),
        );

        instance
    }

    pub fn new(lua_lib: StdLib, lua_option: LuaOption) -> Self {
        // todo issue with lua_lib and lua_option cause ffi error
        // let lua =
        //    mlua::Lua::new_with(lua_lib.into(),
        // lua_option.into()).unwrap_or(mlua::Lua::new());


        let instance = Self {
            context: mlua::Lua::new(),
            gc_mode: mlua::GCMode::Generational,
        };

        instance.gc_gen(
            GenerationalMultiplier::new::<20>(),
            GenerationalMultiplier::new::<100>(),
        );

        instance
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
    ) -> Function<'lua> {
        let lua_func = self
            .context
            .create_function(move |_, parameter: A| Ok(function.call(parameter)))
            .unwrap();


        lua_func
    }

    pub fn create_mut_function<
        A: 'static + mlua::FromLuaMulti<'lua>,
        R: 'static + mlua::ToLuaMulti<'lua>,
        T: 'static + System<'static, A, R> + FnMut<A, Output = R>,
    >(
        &'lua self,
        mut function: T,
    ) -> Function<'lua> {
        let mut_lua_fn = self
            .context
            .create_function_mut(move |_, parameter: A| Ok(function.call_mut(parameter)))
            .unwrap();


        mut_lua_fn
    }

    pub fn bind_fn<T: mlua::ToLua<'lua>>(
        &'lua self,
        func: T,
        lua_fn_name: &str,
    ) -> mlua::Result<()> {
        self.context.globals().raw_set(lua_fn_name, func)?;

        Ok(())
    }

    pub fn parallelize(
        &'lua self,
        lua_fn: mlua::Function<'lua>,
    ) -> mlua::Result<mlua::Thread<'lua>> {
        self.context.create_thread(lua_fn)
    }
}

impl LuaInstance {
    pub fn gc_gen(
        &self,
        minor_value: GenerationalMultiplier<20, 200>,
        major_value: GenerationalMultiplier<100, 1000>,
    ) -> mlua::GCMode {
        self.context.gc_gen(
            std::os::raw::c_int::from(minor_value.multiplier),
            std::os::raw::c_int::from(major_value.multiplier),
        )
    }
}


#[cfg(test)]
mod test {
    use crate::{GenerationalMultiplier, LuaInstance, System};
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

    fn add_10000(x: i32) -> i32 {
        x + 10000
    }

    #[test]
    fn rust_fn_call_test() {
        let lua = LuaInstance::default();

        lua.bind_fn(lua.create_function(times_two), "times_two");

        lua.bind_fn(lua.create_function(add_two), "add_two");

        lua.context
            .load(&std::fs::read_to_string("./lua_src/fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }

    #[test]
    fn rust_multi_parameter_fn_test() {
        let lua = LuaInstance::default();

        lua.bind_fn(lua.create_function(power_of), "power_of");

        lua.context
            .load(&std::fs::read_to_string("./lua_src/multi_param_fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }


    #[test]
    fn rust_mut_fn_test() {
        let lua = LuaInstance::default();


        lua.bind_fn(lua.create_mut_function(modify_x), "modify_x");

        lua.context
            .load(&std::fs::read_to_string("./lua_src/mut_fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }


    #[test]
    fn rust_thread_fn_call_test() {
        let lua = LuaInstance::default();

        let lua_fn = lua.create_function(add_10000);

        let lua_thread = lua.parallelize(lua_fn).unwrap();

        lua.bind_fn(lua_thread, "add_10000");

        lua.context
            .load(&std::fs::read_to_string("./lua_src/thread_call.lua").unwrap())
            .exec()
            .unwrap();
    }
}
