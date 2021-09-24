// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

mod container;
mod contract;

pub use container::*;
pub use contract::*;


#[cfg(test)]
mod tests {
    use crate::AudioClip;
    use std::io::Read;

    #[test]
    fn it_works() {
        // ------------------- initial set up-------------------

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        //--------------------- Loading the File ----------------

        let builder = ambisonic::AmbisonicBuilder::default().build();

        let mut a = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        a.push_str(cargo_dir);
        a.push_str("/Death Is Just Another Path.mp3");

        let mut file = std::fs::File::open(a.as_str()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        //---------------------- Creating the Clip ------------------

        let audio_clip = AudioClip::new(buffer);

        // --------------------- Creating the source-------------------------


        let with_delay = audio_clip.to_ambisonic_buffer();

        let mut a = builder.play_at(with_delay, [50., 1., 0.]);

        a.set_velocity([-10.0, 0.0, 0.0]);

        for i in 0..1000 {
            a.adjust_position([50.0 - i as f32 / 10.0, 1.0, 0.0]);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        a.set_velocity([0.0, 0.0, 0.0]);

        //------------------- Sleep and play ----------------------------

        // sink.sleep_until_end();
    }
}
