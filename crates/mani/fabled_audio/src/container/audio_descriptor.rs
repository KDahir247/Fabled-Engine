// todo don't like the booleans
#[derive(Copy, Clone, Debug)]
pub struct AudioDescriptor {
    pub play_on_awake: bool,
    // pub loopable: bool,
    pub speed_factor: f32,
}
