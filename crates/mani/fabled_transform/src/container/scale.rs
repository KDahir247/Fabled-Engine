use crate::ScaleType;

#[derive(Copy, Clone, Debug, Default)]
pub struct Scale {
    pub value: ScaleType,
}

impl shipyard::Component for Scale {
    type Tracking = shipyard::track::All;
}
