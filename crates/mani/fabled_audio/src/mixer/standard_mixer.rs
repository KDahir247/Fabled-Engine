// This design will follow the spatial mixer

use crate::RawClip;

pub struct StandardMixer<T: rodio::Source<Item = f32>>(T);

impl<T: rodio::Source<Item = f32>> StandardMixer<T> {
    pub fn new(raw_clip: RawClip<T>) -> Self {
        Self { 0: raw_clip.get() }
    }

    // create the transformation for the clip here
}
