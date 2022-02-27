use crate::color::contract::ColorSpace;

pub struct HSL {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}


impl ColorSpace for HSL {
    type TargetSpace = ();

    fn convert_space_from(_: Self::TargetSpace) -> Self {
        todo!()
    }
}
