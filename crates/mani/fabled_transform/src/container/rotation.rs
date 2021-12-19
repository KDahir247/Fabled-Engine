#[derive(Copy, Clone, Debug)]
pub struct Rotation {
    pub value: [f32; 4],
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            value: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl From<[f32; 4]> for Rotation {
    fn from(rot: [f32; 4]) -> Self {
        Self { value: rot }
    }
}

impl shipyard::Component for Rotation {
    type Tracking = shipyard::track::All;
}
