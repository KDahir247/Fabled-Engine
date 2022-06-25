use crate::{AudioClip, RawAmbisonicClip, RawClip};
use mlua::{MetaMethod, UserDataFields, UserDataMethods};

impl mlua::UserData for AudioClip<f32> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {
        _fields.add_field_method_get("channel", |_, audio_clip| Ok(audio_clip.channel));

        _fields.add_field_method_get("samples", |_, audio_clip| {
            let clip_length_milli_sec = audio_clip.duration.unwrap_or_default().as_millis() as u64;

            // should multiply by channels
            Ok(clip_length_milli_sec * audio_clip.sample_rate as u64 / 1000)
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

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_method_mut("standard", |_, audio_clip, ()| {
            let clip = std::mem::take(audio_clip);

            Ok(RawClip::from(clip))
        });


        _methods.add_method_mut("ambisonic", |_, audio_clip, ()| {
            let clip = std::mem::take(audio_clip);


            Ok(RawAmbisonicClip::from(clip))
        });

        _methods.add_meta_function(MetaMethod::ToString, |_, ()| Ok("Native Audio Clip"));

        _methods.add_meta_method(MetaMethod::Len, |_, audio_clip, ()| {
            Ok(audio_clip.audio_data.len())
        });

        _methods.add_meta_method(
            MetaMethod::Index,
            |_lua_context, audio_clip, index: usize| {
                // lua array index start at 1 while rust start at 0. (remapping)
                let lua_offset_index = index - 1;

                let buffer_slice = audio_clip.audio_data.as_slice();

                let mut index_res: mlua::Result<f32> = Err(mlua::Error::RuntimeError(
                    "indexing outside array length".to_string(),
                ));


                if lua_offset_index < buffer_slice.len() {
                    index_res = Ok(buffer_slice[lua_offset_index])
                }

                index_res
            },
        );
    }
}


#[cfg(test)]
mod audio_clip_binding_test {
    use crate::AudioClip;
    use fabled_binding::LuaInstance;

    fn audio_clip(path: String, play_on_awake: bool) -> AudioClip<f32> {
        let file = std::fs::File::open(path).unwrap();

        AudioClip::from_file(file, play_on_awake).unwrap()
    }

    #[test]
    fn native_audio_test() {
        let lua_instance = LuaInstance::default();

        let clip = lua_instance.create_function(audio_clip);

        lua_instance.bind_fn(clip, "audio_clip").unwrap();

        lua_instance.run_script("./lua_src/audio_clip.lua").unwrap();
    }
}
