use crate::AudioListener;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};

impl mlua::UserData for AudioListener {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {
        _fields.add_field_method_get("left_ear", |lua_context, audio_listener| {
            Ok(audio_listener.stereo_left_position)
        });

        _fields.add_field_method_get("right_ear", |_, audio_listener| {
            Ok(audio_listener.stereo_right_position)
        });

        _fields.add_field_method_set(
            "left_ear",
            |_, audio_listener, (left_ear_pos): ([f32; 3])| {
                audio_listener.stereo_left_position = left_ear_pos;
                Ok(())
            },
        );

        _fields.add_field_method_set(
            "right_ear",
            |_, audio_listener, (right_ear_pos): ([f32; 3])| {
                audio_listener.stereo_right_position = right_ear_pos;
                Ok(())
            },
        )
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_meta_function(MetaMethod::ToString, |_, ()| Ok("Audio Listener"))
    }
}


// todo remove.
fn audio_listener(_nil: ()) -> AudioListener {
    AudioListener::default()
}

#[cfg(test)]
mod audio_binding_test {
    use crate::binding::audio_listener_binding::audio_listener;
    use crate::AudioClip;
    use fabled_binding::LuaInstance;


    #[test]
    fn audio_test() {
        let lua_instance = LuaInstance::default();

        let clip = lua_instance.create_function(audio_listener);

        lua_instance.bind_fn(clip, "audio_listener").unwrap();

        lua_instance
            .run_script("./lua_src/audio_listener.lua")
            .unwrap();
    }
}
