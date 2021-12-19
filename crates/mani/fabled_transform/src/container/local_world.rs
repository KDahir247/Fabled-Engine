#[derive(Copy, Clone, Debug)]
pub struct LocalToWorld {
    pub value: [f32; 16],
}

impl Default for LocalToWorld {
    fn default() -> Self {
        Self {
            value: [
                1.0, 0.0, 0.0, 0.0, // col 0
                0.0, 1.0, 0.0, 0.0, // col 1
                0.0, 0.0, 1.0, 0.0, // col 2
                0.0, 0.0, 0.0, 1.0, // col 3
            ],
        }
    }
}

impl shipyard::Component for LocalToWorld {
    type Tracking = shipyard::track::All;
}
