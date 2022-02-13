use shipyard::Component;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Child {
    pub value: u64,
}

impl Component for Child {
    type Tracking = shipyard::track::All;
}
