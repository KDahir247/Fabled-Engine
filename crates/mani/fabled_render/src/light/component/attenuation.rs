use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum FallOffAlgorithm {
    // Similar to unreal engine 4 falloff function
    // 1.0 / (x * x + 1.0)
    BakeryFallOff,
    // Similar to  Frostbite and Unity's HDRP
    // 1.0 / max(x * x, s * s)
    // where s is light size. which will be hardcoded to 0.01
    ClampedUnFiltered,
    // custom bakery falloff where if light_size is 0.01 it will be similar to ClampedUnFiltered
    // and 1 will be BakerFallOff and 0 will be the standard inverse square attenuation.
    // 1.0 / (x * x + s * s)
    // where s is light size
    CustomBakeryFallOff { light_size: f32 },
}

impl Display for FallOffAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone)]
pub struct AttenuationFallOff {
    pub algorithm: FallOffAlgorithm,
}


impl Component for FallOffAlgorithm {
    type Tracking = Modification;
}


impl Display for AttenuationFallOff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Attenuation({})", self.algorithm)
    }
}
