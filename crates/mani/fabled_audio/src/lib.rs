// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

mod config;
mod container;
mod contract;

pub use config::*;
pub use container::*;
pub use contract::*;


#[cfg(test)]
mod tests {
    use crate::{AudioClip, AudioListener, AudioSpatialOutput};
    use std::io::Read;

    #[test]
    fn it_works() {
        // ------------------- initial set up-------------------

        //--------------------- Loading the File ----------------

        ambisonic::AmbisonicBuilder::new();
        let output = AudioSpatialOutput::default();

        let mut a = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        a.push_str(cargo_dir);
        a.push_str("/RSE_pokecenter.mp3");

        let mut file = std::fs::File::open(a.as_str()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();


        let mut a = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        a.push_str(cargo_dir);
        a.push_str("/Peractorum.mp3");

        let mut file = std::fs::File::open(a.as_str()).unwrap();
        let mut buffer1 = Vec::new();
        file.read_to_end(&mut buffer1).unwrap();
        //---------------------- Creating the Clip ------------------

        let audio_clip = AudioClip::from_file(buffer);
        let _audio_clip2 = AudioClip::from_file(buffer1);

        // --------------------- Creating the source-------------------------

        // let clip2 = audio_clip2
        // .to_ambisonic_buffer()
        // .fade_in(std::time::Duration::from_secs(5));

        let source = output.play_at(audio_clip, [10.0, 0.0, 0.0]);


        std::thread::sleep(std::time::Duration::from_secs(1000));
        // sink.append(mixer.0);
        // sink.sleep_until_end();
        //------------------- Sleep and play ----------------------------
        // sink.sleep_until_end();
    }
}
