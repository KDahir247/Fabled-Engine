#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioListener {
    pub position: [f32; 3],
    pub stereo_left_position: [f32; 3],
    pub stereo_right_position: [f32; 3],
}

impl Default for AudioListener {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            stereo_left_position: [-1.0, 0.0, 0.0],
            stereo_right_position: [1.0, 0.0, 0.0],
        }
    }
}
