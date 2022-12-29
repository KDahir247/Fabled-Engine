use fabled_component::{All, Component};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
enum Mode {
    Baked,
    Dynamic,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


#[derive(Copy, Clone)]
pub struct LightMode {
    pub mode: Mode,
}


impl Component for LightMode {
    type Tracking = All;
}

impl Display for LightMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!("LightMode(Mode : {})", self.mode)
    }
}
