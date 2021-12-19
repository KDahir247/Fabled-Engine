#[derive(Copy, Clone, Debug)]
pub struct Translation {
    pub value: [f32; 4],
}


impl Default for Translation {
    fn default() -> Self {
        Self {
            value: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl shipyard::Component for Translation {
    type Tracking = shipyard::track::All;
}
