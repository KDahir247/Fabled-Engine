use fabled_component::{All, Component};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// Mesh with channel0 will only get illuminated by light source that are in
// channel0 and so on.
#[derive(Copy, Clone)]
pub struct LightChannel {
    value: Channel,
}

impl Component for LightChannel {
    type Tracking = All;
}

impl Display for LightChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LightChannel(channel : {})", self.value)
    }
}
