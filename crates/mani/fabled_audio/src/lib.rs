// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

mod container;
mod contract;

pub use container::*;
pub use contract::*;


#[cfg(test)]
mod tests {
    use crate::AudioClip;
    use rodio::Source;
    use std::io::Read;

    #[test]
    fn it_works() {
        // ------------------- initial set up -------------------

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        //--------------------- Loading the File ----------------

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

        let with_delay = audio_clip.to_buffer().buffered().repeat_infinite();

        sink.append(with_delay);

        //------------------- Sleep and play ----------------------------

        sink.sleep_until_end();
    }
}
