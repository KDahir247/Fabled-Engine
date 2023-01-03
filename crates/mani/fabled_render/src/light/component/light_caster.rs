use fabled_component::{All, Component};
use std::fmt::{Display, Formatter};


// mesh object cast shadow if this is added
#[derive(Copy, Clone)]
pub struct ShadowCaster;

// mesh object will receive shadow if this is added.
#[derive(Copy, Clone)]
pub struct ShadowReceiver;


impl Component for ShadowCaster {
    type Tracking = All;
}

impl Component for ShadowReceiver {
    type Tracking = All;
}

impl Display for ShadowCaster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ShadowCaster")
    }
}

impl Display for ShadowReceiver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ShadowReceiver")
    }
}
