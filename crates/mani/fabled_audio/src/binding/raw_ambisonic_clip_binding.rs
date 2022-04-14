use crate::RawAmbisonicClip;
use ambisonic::rodio::Source;
use mlua::{UserDataFields, UserDataMethods};

impl mlua::UserData for RawAmbisonicClip {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(_fields: &mut F) {
        _fields.add_field_method_get("channel", |_, raw_ambisonic_clip| {
            Ok(raw_ambisonic_clip.dyn_clip.channels())
        });

        _fields.add_field_method_get("samples", |_, raw_ambisonic_clip| {
            let clip_len_millisec = raw_ambisonic_clip
                .dyn_clip
                .total_duration()
                .unwrap_or_default()
                .as_millis() as u64;

            Ok(clip_len_millisec * raw_ambisonic_clip.dyn_clip.sample_rate() as u64 / 1000)
        });

        _fields.add_field_method_get("sample_rate", |_, raw_ambisonic_clip| {
            Ok(raw_ambisonic_clip.dyn_clip.sample_rate())
        });

        _fields.add_field_method_get("duration", |lua, raw_ambisonic_clip| {
            let duration = raw_ambisonic_clip
                .dyn_clip
                .total_duration()
                .unwrap_or_default();
            let table = lua.create_table_with_capacity(3, 0).unwrap_or_default();

            unsafe {
                table
                    .raw_set("seconds", duration.as_secs())
                    .unwrap_unchecked();
                table
                    .raw_set("milliseconds", duration.as_millis())
                    .unwrap_unchecked();
                table
                    .raw_set("microseconds", duration.as_millis())
                    .unwrap_unchecked();
            }

            Ok(table)
        });
    }


    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        _methods.add_method_mut("low_pass", |_, raw_ambisonic_clip, (frequency): u32| {
            let moved_raw_ambisonic_clip = std::mem::take(raw_ambisonic_clip);

            Ok(moved_raw_ambisonic_clip.low_pass(frequency))
        });

        _methods.add_method_mut("buffered", |_, raw_ambisonic_clip, ()| {
            let moved_raw_ambisonic_clip = std::mem::take(raw_ambisonic_clip);

            Ok(moved_raw_ambisonic_clip.buffered())
        });

        _methods.add_method_mut(
            "delay",
            |_, raw_ambisonic_clip, (seconds, micro_seconds): (u64, u32)| {
                let moved_raw_ambisonic_clip = std::mem::take(raw_ambisonic_clip);

                Ok(moved_raw_ambisonic_clip.delay(seconds, micro_seconds))
            },
        );

        _methods.add_method_mut(
            "fade_in",
            |_, raw_ambisonic_clip, (seconds, micro_seconds): (u64, u32)| {
                let moved_raw_ambisonic_clip = std::mem::take(raw_ambisonic_clip);

                Ok(moved_raw_ambisonic_clip.fade_in(seconds, micro_seconds))
            },
        );

        _methods.add_method_mut("loop", |_, raw_ambisonic_clip, ()| {
            let moved_raw_ambisonic_clip = std::mem::take(raw_ambisonic_clip);

            Ok(moved_raw_ambisonic_clip.repeat())
        });

        _methods.add_method_mut("speed", |_, raw_ambisonic_clip, (factor): (f32)| {
            let moved_ambisonic_clip = std::mem::take(raw_ambisonic_clip);

            Ok(moved_ambisonic_clip.speed(factor))
        });
    }
}
