// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

mod config;
mod container;
mod contract;
mod source;

pub use config::*;
pub use container::*;
pub use contract::*;
pub use source::*;


#[cfg(test)]
mod tests {
    use crate::{Ambisonic, AudioClip, AudioSpatialOutput};
    use std::io::Read;

    #[test]
    fn it_works() {
        // ------------------- initial set up-------------------

        //--------------------- Loading the File ----------------

        ambisonic::AmbisonicBuilder::new();
        let output = AudioSpatialOutput::default();

        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str("/RSE_pokecenter.mp3");

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        //---------------------- Creating the Clip ------------------

        let audio_clip = AudioClip::from_file(buffer);

        // --------------------- Creating the source-------------------------

        let raw_clip = Ambisonic::from(audio_clip);

        let source = output.play_at(raw_clip, [10.0, 0.0, 0.0]);


        std::thread::sleep(std::time::Duration::from_secs(1000));
    }
}
