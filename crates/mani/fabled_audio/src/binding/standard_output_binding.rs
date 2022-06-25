use crate::StandardOutput;
use mlua::UserDataFields;

impl mlua::UserData for StandardOutput<f32> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {}

    // fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
    //     _methods.add_method("play", |_, audio_source, clip: RawClip<f32>|
    // Ok(())); }
}

#[cfg(test)]
mod standard_output_binding_test {
    use crate::{AudioClip, AudioListener, RawClip, StandardOutput};
    use fabled_binding::LuaInstance;

    fn raw_audio_clip(path: String, play_on_awake: bool) -> RawClip<f32> {
        let file = std::fs::File::open(path).unwrap();

        let native_audio_clip = AudioClip::from_file(file, play_on_awake).unwrap();

        RawClip::from(native_audio_clip)
    }

    fn create_audio_source(_: ()) -> StandardOutput<f32> {
        StandardOutput::new(AudioListener::default())
    }

    #[test]
    fn standard_output_test() {
        let lua_instance = LuaInstance::default();

        let raw_clip = lua_instance.create_function(raw_audio_clip);

        let audio_source = lua_instance.create_function(create_audio_source);

        lua_instance.bind_fn(raw_clip, "raw_audio_clip").unwrap();

        lua_instance.bind_fn(audio_source, "audio_source").unwrap();

        lua_instance
            .run_script("./lua_src/standard_output.lua")
            .unwrap();
    }
}
