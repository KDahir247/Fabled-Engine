// This will be similar to audio_spatial_output, except that the output audio
// will be non spatial and will not play the clip via ambisonic

use crate::RawClip;

pub struct AudioOutput {}


impl AudioOutput {
    pub fn play<T>(&self, clip: RawClip<T>)
    where
        T: rodio::Source<Item = i16> + Send + 'static, {
    }
}
