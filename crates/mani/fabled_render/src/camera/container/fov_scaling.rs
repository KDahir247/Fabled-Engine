use crate::camera::{Aperture, AspectRatio, FishLens};

#[derive(Copy, Clone, PartialEq)]
pub enum FovScalingAlgorithm {
    HorizontalPlus {
        target_aspect: AspectRatio,
        current_aspect: AspectRatio,
    },
    Anamorphic {
        len_type: FishLens,
        frame_aperture: Aperture,
        //
        anamorphic_descriptor: AnamorphicDescriptor,
    },
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AnamorphicDescriptor {
    pub sensor_aspect_ratio: AspectRatio,
    pub single_focus_solution: AnamorphicLen,
    // none is 1 for full frame
    pub crop_factor: Option<f32>,
    // none is 1 which is no focal reducer.
    pub focal_reducer: Option<f32>,
    pub focal_length: f32,
    pub anamorphic_adapter: f32,
    // between 0 and 100
    pub focus_distance: f32,
}

impl Default for AnamorphicDescriptor {
    fn default() -> Self {
        AnamorphicDescriptor {
            sensor_aspect_ratio: Default::default(),
            single_focus_solution: Default::default(),
            crop_factor: None,
            focal_reducer: Some(0.71),
            focal_length: 50.0,
            anamorphic_adapter: 1.5,
            focus_distance: 50.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct AnamorphicLen(pub f32);

impl AnamorphicLen {
    pub const NONE: AnamorphicLen = AnamorphicLen(1.0);
    pub const SLR_MAGIC_RANGEFINDER: AnamorphicLen = AnamorphicLen(0.99);
    pub const FM_LENS: AnamorphicLen = AnamorphicLen(0.93);
    pub const RECTILUX_3FF: AnamorphicLen = AnamorphicLen(0.88);
    pub const RECTILUX_CORE_DNA: AnamorphicLen = AnamorphicLen(0.91);
}

impl AnamorphicLen {
    pub const fn custom(val: f32) -> AnamorphicLen {
        AnamorphicLen(val)
    }
}

impl Default for AnamorphicLen {
    fn default() -> Self {
        AnamorphicLen::NONE
    }
}
