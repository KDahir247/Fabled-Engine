// this will substitute data for audio_clip from_file function
#[derive(Debug)]
pub enum AudioType {
    Packed(Vec<u8>),
    Loose(std::fs::File),
}
