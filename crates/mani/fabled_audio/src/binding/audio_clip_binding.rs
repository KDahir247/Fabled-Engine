use crate::{AmbisonicOutput, AudioClip, RawAmbisonicClip, RawClip, StandardOutput};
use mlua::{Lua, MetaMethod, MultiValue, UserDataFields, UserDataMethods};
use rodio::Source;

impl mlua::UserData for AudioClip<f32> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {
        _fields.add_field_method_get("channel", |_, audio_clip| Ok(audio_clip.channel));

        _fields.add_field_method_get("samples", |_, audio_clip| {
            let clip_length_sec = audio_clip.duration.unwrap_or_default().as_millis() as u64;

            // should multiply by channels
            Ok(clip_length_sec * audio_clip.sample_rate as u64 / 1000)
        });

        _fields.add_field_method_get("sample_rate", |_, audio_clip| Ok(audio_clip.sample_rate));

        _fields.add_field_method_get("frame_length", |_, audio_clip| {
            Ok(audio_clip.current_frame_len.unwrap_or_default())
        });

        // todo currently mp3 and vorbis will result in 0 for duration (mp3 and ogg)
        _fields.add_field_method_get("duration", |lua, audio_clip| {
            let duration = audio_clip.duration.unwrap_or_default();
            let table = lua.create_table_with_capacity(3, 0).unwrap();

            unsafe {
                table
                    .raw_set("seconds", duration.as_secs())
                    .unwrap_unchecked();
                table
                    .raw_set("milliseconds", duration.as_millis())
                    .unwrap_unchecked();
                table
                    .raw_set("microseconds", duration.as_micros())
                    .unwrap_unchecked();
            }
            Ok(table)
        });


        _fields.add_field_method_get("buffer", |lua, audio_clip| {
            lua.create_sequence_from(audio_clip.audio_data.clone())
        });
    }

    // todo remove
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_method("play", |_, audio, (volume): (f32)| {
            let clip = AudioClip {
                audio_data: audio.audio_data.clone(),
                duration: audio.duration,
                current_frame_len: audio.current_frame_len,
                sample_rate: audio.sample_rate,
                channel: audio.channel,
            };

            let raw_clip = RawClip::from(clip);

            let stand_output = StandardOutput::default();

            stand_output.play_omni(raw_clip, volume);

            std::thread::sleep(std::time::Duration::from_secs(10));
            Ok(stand_output)
        });


        _methods.add_method("standard", |_, audio_clip, ()| {
            let clip = AudioClip {
                audio_data: audio_clip.audio_data.clone(),
                duration: audio_clip.duration,
                current_frame_len: audio_clip.current_frame_len,
                sample_rate: audio_clip.sample_rate,
                channel: audio_clip.channel,
            };

            Ok(RawClip::from(clip))
        });

        _methods.add_method("ambisonic", |_, audio_clip, ()| {
            let clip = AudioClip {
                audio_data: audio_clip.audio_data.clone(),
                duration: audio_clip.duration,
                current_frame_len: audio_clip.current_frame_len,
                sample_rate: audio_clip.sample_rate,
                channel: audio_clip.channel,
            };


            Ok(RawAmbisonicClip::from(clip))
        });

        _methods.add_meta_function(MetaMethod::ToString, |_, ()| Ok("Audio Clip"));
    }
}

fn audio_clip(path: String, play_on_awake: bool) -> AudioClip<f32> {
    let file = std::fs::File::open(path).unwrap();

    AudioClip::from_file(file, play_on_awake).unwrap()
}

#[cfg(test)]
mod audio_binding_test {
    use crate::binding::audio_clip_binding::audio_clip;
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
