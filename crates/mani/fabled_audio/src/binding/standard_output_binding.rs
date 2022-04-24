use crate::StandardOutput;
use mlua::{UserDataFields, UserDataMethods};

impl mlua::UserData for StandardOutput<f32> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {
        
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {

    }
}
