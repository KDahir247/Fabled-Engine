use crate::RawClip;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use rodio::Source;

impl mlua::UserData for RawClip<f32> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {
        _fields.add_field_method_get("channel", |_, raw_clip| Ok(raw_clip.dyn_clip.channels()));

        _fields.add_field_method_get("samples", |_, raw_clip| {
            let clip_len_milli_sec = raw_clip
                .dyn_clip
                .total_duration()
                .unwrap_or_default()
                .as_millis() as u64;

            // should multiply by channels
            Ok(clip_len_milli_sec * raw_clip.dyn_clip.sample_rate() as u64 / 1000)
        });

        _fields.add_field_method_get("sample_rate", |_, raw_clip| {
            Ok(raw_clip.dyn_clip.sample_rate())
        });

        _fields.add_field_method_get("frame_length", |_, raw_clip| {
            Ok(raw_clip.dyn_clip.current_frame_len().unwrap_or_default())
        });

        // todo currently mp3 and vorbis will result in 0 for duration (mp3 and ogg)
        _fields.add_field_method_get("duration", |lua, raw_clip| {
            let duration = raw_clip.dyn_clip.total_duration().unwrap_or_default();
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
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_meta_function(MetaMethod::ToString, |_, ()| Ok("Standard Audio Clip"));

        _methods.add_method_mut("low_pass", |_, raw_clip, (frequency): u32| {
            let moved_raw_clip = std::mem::take(raw_clip);

            Ok(moved_raw_clip.low_pass(frequency))
        });

        _methods.add_method_mut("buffered", |_, raw_clip, ()| {
            let moved_raw_clip = std::mem::take(raw_clip);

            Ok(moved_raw_clip.buffered())
        });

        _methods.add_method_mut(
            "delay",
            |_, raw_clip, (seconds, micro_seconds): (u64, u32)| {
                let moved_raw_clip = std::mem::take(raw_clip);

                Ok(moved_raw_clip.delay(seconds, micro_seconds))
            },
        );

        _methods.add_method_mut(
            "fade_in",
            |_, raw_clip, (seconds, micro_seconds): (u64, u32)| {
                let moved_raw_clip = std::mem::take(raw_clip);

                Ok(moved_raw_clip.fade_in(seconds, micro_seconds))
            },
        );

        _methods.add_method_mut(
            "reverb",
            |_, raw_clip, (seconds, micro_seconds, amplitude): (u64, u32, f32)| {
                let moved_raw_clip = std::mem::take(raw_clip);

                Ok(moved_raw_clip.reverb(seconds, micro_seconds, amplitude))
            },
        );

        _methods.add_method_mut("loop", |_, raw_clip, ()| {
            let moved_raw_clip = std::mem::take(raw_clip);

            Ok(moved_raw_clip.repeat())
        });

        _methods.add_method_mut("speed", |_, raw_clip, (factor): (f32)| {
            let moved_raw_clip = std::mem::take(raw_clip);

            Ok(moved_raw_clip.speed(factor))
        });

        _methods.add_function("is_ambisonic", |_, ()| Ok(false))
    }
}
