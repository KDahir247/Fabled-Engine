use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct Temperature {
    pub kelvin: f32,
}


impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Temperature(Kelvin : {})", self.kelvin)
    }
}

// When temperature change we will do adaption of previous temp and current
// temp.
impl Component for Temperature {
    type Tracking = Modification;
}
