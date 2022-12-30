#[derive(Copy, Clone)]
pub struct Efficacy {
    pub high_lumen_per_watt: f32,
    pub low_lumen_per_watt: f32,
}

impl Efficacy {
    pub const INCANDESCENT: Efficacy = Efficacy {
        high_lumen_per_watt: 18.0,
        low_lumen_per_watt: 12.0,
    };

    pub const LED: Efficacy = Efficacy {
        high_lumen_per_watt: 90.0,
        low_lumen_per_watt: 30.0,
    };

    pub const HALOGEN: Efficacy = Efficacy {
        high_lumen_per_watt: 24.0,
        low_lumen_per_watt: 16.0,
    };

    pub const CFL: Efficacy = Efficacy {
        high_lumen_per_watt: 60.0,
        low_lumen_per_watt: 40.0,
    };

    pub const FLUORESCENT: Efficacy = Efficacy {
        high_lumen_per_watt: 75.0,
        low_lumen_per_watt: 45.0,
    };
}

