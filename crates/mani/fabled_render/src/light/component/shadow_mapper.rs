use fabled_component::{All, Component};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct ShadowMapper {
    pub depth_bias: f32,
    pub normal_bias: f32,
    pub resolution: f32,
}


impl Default for ShadowMapper {
    fn default() -> Self {
        ShadowMapper {
            depth_bias: 0.02,
            normal_bias: 0.6,
            resolution: 4096.0,
        }
    }
}

impl Display for ShadowMapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ShadowMapper(\n\tdepth bias : {},\n\tnormal bias : {}\n\tresolution : {}\n)",
            self.depth_bias, self.normal_bias, self.resolution
        )
    }
}

impl Component for ShadowMapper {
    type Tracking = All;
}
