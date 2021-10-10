use minimp3::Frame;

#[derive(Default)]
pub struct Mp3Reader;

impl Mp3Reader {
    pub fn open_mp3<P: AsRef<std::path::Path>>(&self, mp3_path: P) {}
}


#[cfg(test)]
mod mp3_decoding_test {
    use crate::Mp3Reader;

    #[test]
    fn decoding_file() {
        //
        let mp3_reader = Mp3Reader::default();

        let mp3_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/epic.mp3"].join("");

        mp3_reader.open_mp3(mp3_path);
    }
}
