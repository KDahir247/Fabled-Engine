use crate::{ByteType, GCMultiplier, LuaBindingError, LuaOption, MemorySize, StdLib, System};
use mlua::{ExternalError, FromLuaMulti, Function, GCMode, ToLuaMulti};
use std::env::Args;
use std::error::Error;
use std::fmt::Debug;
use std::future::Future;

#[repr(align(32))]
pub struct LuaInstance(mlua::Lua);

impl Default for LuaInstance {
    fn default() -> Self {
        LuaInstance::new(StdLib::ALL_SAFE, LuaOption::new())
    }
}

impl LuaInstance {
    pub unsafe fn new_unsafe(lua_lib: mlua::StdLib, lua_option: LuaOption) -> Self {
        let mut instance = Self {
            0: mlua::Lua::unsafe_new_with(lua_lib, lua_option.into()),
        };

        instance.0.set_app_data(mlua::GCMode::Generational);

        set_warning_lint(&instance.0);

        instance
            .0
            .set_named_registry_value("gc_pause", 200)
            .unwrap_unchecked();

        instance
            .0
            .set_named_registry_value("gc_step_multiplier", 100)
            .unwrap_unchecked();

        instance
            .0
            .set_named_registry_value("memory_limit", 0)
            .unwrap_unchecked();

        instance.set_memory_limit(MemorySize::new(0, ByteType::Byte));

        instance.gc_generational(GCMultiplier::new::<20>(), GCMultiplier::new::<100>());

        instance
    }

    pub fn new(lua_lib: mlua::StdLib, lua_option: LuaOption) -> Self {
        let lua = mlua::Lua::new_with(lua_lib, lua_option.into()).unwrap_or(mlua::Lua::new());

        let mut instance = Self { 0: lua };

        instance.0.set_app_data(mlua::GCMode::Generational);

        set_warning_lint(&instance.0);

        unsafe {
            instance
                .0
                .set_named_registry_value("gc_pause", 200)
                .unwrap_unchecked();

            instance
                .0
                .set_named_registry_value("gc_step_multiplier", 100)
                .unwrap_unchecked();

            instance
                .0
                .set_named_registry_value("memory_limit", 0)
                .unwrap_unchecked();
        }

        instance.set_memory_limit(MemorySize::new(0, ByteType::Byte));

        instance.gc_generational(GCMultiplier::new::<20>(), GCMultiplier::new::<100>());

        instance
    }
}

impl<'lua> LuaInstance {
    pub fn create_function<
        A: mlua::FromLuaMulti<'lua>,
        R: mlua::ToLuaMulti<'lua>,
        T: 'static + System<A, R> + Fn<A, Output = R>,
    >(
        &'lua self,
        function: T,
    ) -> Function<'lua> {
        let lua_func = self
            .0
            .create_function(move |_, parameter: A| Ok(function.call(parameter)))
            .unwrap();


        lua_func
    }

    pub fn create_mut_function<
        A: mlua::FromLuaMulti<'lua>,
        R: mlua::ToLuaMulti<'lua>,
        T: 'static + System<A, R> + FnMut<A, Output = R>,
    >(
        &'lua self,
        mut function: T,
    ) -> Function<'lua> {
        let mut_lua_fn = self
            .0
            .create_function_mut(move |_, parameter: A| Ok(function.call_mut(parameter)))
            .unwrap();


        mut_lua_fn
    }


    pub fn create_async_function<
        A: mlua::FromLuaMulti<'lua>,
        R: mlua::ToLuaMulti<'lua>,
        FR: 'lua + Future<Output = mlua::Result<R>>,
        F: 'static + System<A, FR> + Fn<A, Output = FR>,
    >(
        &'lua self,
        function: F,
    ) -> Function<'lua> {
        let async_fn = self
            .0
            .create_async_function(move |_, parameters: A| function.call(parameters))
            .unwrap();

        async_fn
    }

    pub fn bind_fn<T: mlua::ToLua<'lua>>(
        &'lua self,
        func: T,
        lua_fn_name: &str,
    ) -> mlua::Result<()> {
        self.0.globals().raw_set(lua_fn_name, func)?;

        Ok(())
    }

    pub fn parallelize(
        &'lua self,
        lua_fn: mlua::Function<'lua>,
    ) -> mlua::Result<mlua::Thread<'lua>> {
        self.0.create_thread(lua_fn)
    }
}

impl LuaInstance {
    pub fn gc_generational(
        &self,
        minor_value: GCMultiplier<10, 200>,
        major_value: GCMultiplier<50, 1000>,
    ) {
        let mut gc_mode = unsafe { self.0.app_data_mut::<mlua::GCMode>().unwrap_unchecked() };

        *gc_mode = self.0.gc_gen(
            std::os::raw::c_int::from(minor_value.multiplier),
            std::os::raw::c_int::from(major_value.multiplier),
        );
    }


    pub fn gc_incremental(
        &mut self,
        pause_value: GCMultiplier<50, 1000>,
        step_multiplier: GCMultiplier<100, 1000>,
        step_size: GCMultiplier<12, 14>,
    ) {
        let mut gc_mode = unsafe { self.0.app_data_mut::<mlua::GCMode>().unwrap_unchecked() };

        *gc_mode = self.0.gc_inc(
            std::os::raw::c_int::from(pause_value.multiplier),
            std::os::raw::c_int::from(step_multiplier.multiplier),
            std::os::raw::c_int::from(step_size.multiplier),
        );
    }

    pub fn gc_collect(&self) {
        const GC_COLLECT_WARNING: &str = "lua shouldn't directly be possibly to run out of stack space. The only way this error is triggered is if a lua function contains excessive arguments or callback return excessive return value";
        self.0
            .gc_collect()
            .unwrap_or_else(|err| set_error_lint(err))
    }

    pub fn gc_stop(&self) {
        self.0.gc_stop()
    }

    pub fn gc_restart(&self) {
        self.0.gc_restart()
    }

    pub fn gc_set_pause(&self, pause_factor: i32) {
        let gc_pause: i32 = self
            .0
            .named_registry_value("gc_pause")
            .unwrap_or_else(|err| {
                set_error_lint(err);
                0
            });

        if gc_pause.ne(&pause_factor) {
            self.0
                .set_named_registry_value("gc_pause", self.0.gc_set_pause(pause_factor))
                .unwrap_or_else(|err| set_error_lint(err));
        }
    }

    pub fn gc_set_step_multiplier(&self, step_multiplier: i32) {
        let gc_step: i32 = self
            .0
            .named_registry_value("gc_step_multiplier")
            .unwrap_or_else(|err| {
                set_error_lint(err);
                0
            });

        if gc_step.ne(&step_multiplier) {
            self.0
                .set_named_registry_value(
                    "gc_step_multiplier",
                    self.0.gc_set_step_multiplier(step_multiplier),
                )
                .unwrap_or_else(|err| set_error_lint(err));
        }
    }

    pub fn set_memory_limit(&self, limit_size: MemorySize) {
        let previous_memory_limit: usize = self
            .0
            .named_registry_value("memory_limit")
            .unwrap_or_else(|err| {
                set_error_lint(err);
                0
            });

        self.0
            .set_named_registry_value(
                "memory_limit",
                self.0
                    .set_memory_limit(limit_size.unit)
                    .unwrap_or_else(|err| {
                        set_error_lint(err);
                        previous_memory_limit
                    }),
            )
            .unwrap_or_else(|err| set_error_lint(err));
    }
}

// todo not complete requires fabled_logger
fn set_error_lint(lua_err: mlua::Error) {
    let binding_err: LuaBindingError = lua_err.into();

    println!("{}", binding_err);
}

fn set_warning_lint(lua: &mlua::Lua) {
    // todo not complete requires fabled_logger
    lua.set_warning_function(|_lua, warning_msg, continued| {
        println!("{:?}", warning_msg);


        Ok(())
    });

    lua.set_hook(mlua::HookTriggers::every_nth_instruction(500), |lua, debug|{
        lua.warning("Description : lua script cognitive complexity is high! \n Current Size : 600. \n Solution : separate monolithic lua script to other or new lua script", true )
    }).unwrap()
}

#[cfg(test)]
mod test {
    use crate::ByteType::Byte;
    use crate::{ByteType, GCMultiplier, LuaInstance, MemorySize, System};
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

    async fn sleep(n: u64) -> mlua::Result<&'static str> {
        futures_timer::Delay::new(std::time::Duration::from_millis(n)).await;
        Ok("done")
    }

    #[test]
    fn rust_fn_call_test() {
        let lua = LuaInstance::default();

        lua.bind_fn(lua.create_function(times_two), "times_two");

        lua.bind_fn(lua.create_function(add_two), "add_two");

        lua.0
            .load(&std::fs::read_to_string("./lua_src/fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }

    #[test]
    fn rust_multi_parameter_fn_test() {
        let lua = LuaInstance::default();

        lua.bind_fn(lua.create_function(power_of), "power_of");

        lua.0
            .load(&std::fs::read_to_string("./lua_src/multi_param_fn_call.lua").unwrap())
            .exec()
            .unwrap();
    }


    #[test]
    fn rust_mut_fn_test() {
        let lua = LuaInstance::default();

        lua.bind_fn(lua.create_mut_function(modify_x), "modify_x");

        lua.0
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

        lua.0
            .load(&std::fs::read_to_string("./lua_src/thread_call.lua").unwrap())
            .exec()
            .unwrap();
    }


    #[test]
    fn rust_name_registry_test() {
        let mut lua = LuaInstance::default();

        lua.gc_set_step_multiplier(5);
    }

    #[actix_rt::test]
    async fn rust_async_fn_call_test() {
        let lua = LuaInstance::default();

        let sleep_fn = lua.create_async_function(sleep);

        lua.bind_fn(sleep_fn, "sleep");

        let res: mlua::Result<String> = lua.0.load("return sleep(...)").call_async(2500).await;

        println!("{}", res.unwrap());
    }
}
