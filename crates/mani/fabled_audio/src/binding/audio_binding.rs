use crate::{AudioClip, RawClip, StandardOutput};
use ambisonic::rodio::Source;
use mlua::{Lua, MetaMethod, MultiValue, UserDataFields, UserDataMethods};
use std::ops::Deref;

// todo0
impl mlua::UserData for AudioClip<f32> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        unsafe {
            _methods.add_method("play", |_, audio, (volume): (f32)| {
                let clip: AudioClip<f32> = AudioClip {
                    audio_data: audio.audio_data.clone(),
                    duration: audio.duration,
                    current_frame_len: audio.current_frame_len,
                    sample: audio.sample,
                    channel: audio.channel,
                };

                let raw_clip = RawClip::from(clip);

                let stand_output = StandardOutput::default();

                stand_output.play_omni(raw_clip, volume);

                std::thread::sleep(std::time::Duration::from_secs(1000));
                Ok(())
            });
        }

        _methods.add_meta_function(
            MetaMethod::Call,
            |_, (path, play_on_awake): (String, bool)| {
                let file = std::fs::File::open(path).unwrap();

                let clip = AudioClip::from_file(file, play_on_awake).unwrap();

                Ok(clip)
            },
        )
    }
}

fn audio_clip(path: String, play_on_awake: bool) -> AudioClip<f32> {
    let file = std::fs::File::open(path).unwrap();

    AudioClip::from_file(file, play_on_awake).unwrap()
}

#[cfg(test)]
mod audio_binding_test {
    use crate::binding::audio_binding::audio_clip;
    use crate::AudioClip;
    use fabled_binding::LuaInstance;


    #[test]
    fn audio_test() {
        let lua_instance = LuaInstance::default();

        let clip = lua_instance.create_function(audio_clip);

        lua_instance.bind_fn(clip, "audio_clip").unwrap();

        lua_instance
            .0
            .load(&std::fs::read_to_string("./lua_src/create_play_audio.lua").unwrap())
            .exec()
            .unwrap();
    }
}
