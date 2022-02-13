#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Parent {
    pub value: u64,
}

impl shipyard::Component for Parent {
    type Tracking = shipyard::track::All;
}
