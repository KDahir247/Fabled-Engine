pub struct CurveUserParam {
    pub toe_strength: f32,
    pub toe_length: f32,
    pub shoulder_strength: f32,
    pub shoulder_length: f32,
    pub shoulder_angle: f32,
    pub gamma: f32,
}


impl Default for CurveUserParam {
    fn default() -> Self {
        CurveUserParam {
            toe_strength: 0.0,
            toe_length: 0.5,
            shoulder_strength: 0.0,
            shoulder_length: 0.5,
            shoulder_angle: 0.0,
            gamma: 1.0,
        }
    }
}

pub struct CurveParamsDirect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub w: f32,
    pub overshoot_x: f32,
    pub overshoot_y: f32,
    pub gamma: f32,
}

impl Default for CurveParamsDirect {
    fn default() -> Self {
        CurveParamsDirect {
            x0: 0.25,
            y0: 0.25,
            x1: 0.75,
            y1: 0.75,
            w: 1.0,
            overshoot_x: 0.0,
            overshoot_y: 0.0,
            gamma: 1.0,
        }
    }
}

#[derive(Copy, Clone)]
struct CurveSegment {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale_x: f32,
    pub ln_a: f32,
    pub b: f32,
}

impl Default for CurveSegment {
    fn default() -> Self {
        CurveSegment {
            offset_x: 0.0,
            offset_y: 0.0,
            scale_x: 1.0,
            ln_a: 0.0,
            b: 1.0,
        }
    }
}

pub struct FullCurve {
    pub segment: [CurveSegment; 3],
    pub inv_segment: [CurveSegment; 3],
    pub w: f32,
    pub inv_w: f32,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
}


impl Default for FullCurve {
    fn default() -> Self {
        FullCurve {
            segment: [CurveSegment::default(); 3],
            inv_segment: [CurveSegment::default(); 3],
            w: 1.0,
            inv_w: 1.0,
            x0: 0.25,
            x1: 0.75,
            y0: 0.25,
            y1: 0.75,
        }
    }
}
