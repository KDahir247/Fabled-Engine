use fabled_audio::{AmbisonicOutput, AudioClipOld};
use fabled_render::mesh::{Cone, Model};

mod core;
mod test;

fn main() {
    // superluminal_perf::set_current_thread_name("Main_Thread");
    // superluminal_perf::begin_event("Starting");

    let path = &[
        env!("CARGO_MANIFEST_DIR"),
        "/crates/mani/fabled_audio",
        "/src/audio/Deus Ex Tempus.ogg",
    ]
    .join("");
    let file = std::fs::File::open(path).unwrap();
    //---------------------- Creating the Clip ------------------
    let standard_output = AmbisonicOutput::default();

    let audio_clip: AudioClipOld<f32> = AudioClipOld::from_file(file, true).unwrap();

    standard_output.play_omni(audio_clip, 1.);


    std::thread::sleep(std::time::Duration::from_secs(100));
    // superluminal_perf::end_event();

    // core::State::run();
}
