// static
#[derive(Copy, Clone, Debug, Default)]
pub struct Frozen {}

impl shipyard::Component for Frozen {
    // removal, deletion, and insert, so just about all.
    type Tracking = shipyard::track::All;
}
