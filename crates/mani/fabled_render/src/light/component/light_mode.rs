use fabled_component::{All, Component};
use std::fmt::{Display, Formatter};

// Baked Lighting can only cast shadow on static object. Light attenuation will
// be store a texture file prior to passing lighting to shader so it can use the
// texture and sample it to retrieve the attenuation

// Dynamic Lighting will cast on both static object and dynamic object and will
// calculate attenuation dynamically in the shader.

#[derive(Copy, Clone)]
enum Mode {
    Baked,
    Stationary,
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
        writeln!(f, "LightMode(Mode : {})", self.mode)
    }
}
