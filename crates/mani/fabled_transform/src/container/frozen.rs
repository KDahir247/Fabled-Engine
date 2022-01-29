// static
#[derive(Copy, Clone, Debug, Default)]
pub struct Frozen {}


impl shipyard::Component for Frozen {
    type Tracking = shipyard::track::All;
}
