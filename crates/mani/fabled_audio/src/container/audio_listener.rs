#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct AudioListener {
    pub stereo_left_position: [f32; 3],
    pub stereo_right_position: [f32; 3],
}


