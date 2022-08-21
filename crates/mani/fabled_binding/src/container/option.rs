#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuaOption {
    pub catch_rust_panics: bool,
    pub thread_cache_size: usize,
}

impl Default for LuaOption {
    fn default() -> Self {
        LuaOption::new()
    }
}

impl LuaOption {
    pub fn new() -> Self {
        Self {
            catch_rust_panics: true,
            thread_cache_size: 0,
        }
    }

    pub fn new_with_config(thread_cache_size: usize, catch_rust_panics: bool) -> Self {
        Self {
            catch_rust_panics,
            thread_cache_size,
        }
    }
}


impl From<LuaOption> for mlua::LuaOptions {
    fn from(lua_option: LuaOption) -> Self {
        mlua::LuaOptions::new()
            .thread_cache_size(lua_option.thread_cache_size)
            .catch_rust_panics(lua_option.catch_rust_panics)
    }
}
