#[derive(Copy, Clone, Debug)]
pub struct Efficacy {
    pub high_watt_factor: f32,
    pub low_watt_factor: f32,
}


impl Efficacy {
    pub const INCANDESCENT: Efficacy = Efficacy {
        high_watt_factor: 18.0,
        low_watt_factor: 12.0,
    };

    pub const LED: Efficacy = Efficacy {
        high_watt_factor: 90.0,
        low_watt_factor: 30.0,
    };

    pub const HALOGEN: Efficacy = Efficacy {
        high_watt_factor: 24.0,
        low_watt_factor: 16.0,
    };

    pub const FLUORESCENT: Efficacy = Efficacy {
        high_watt_factor: 75.0,
        low_watt_factor: 45.0,
    };
}


pub fn watt_to_lumen(watts: f32, luminous_efficacy: Efficacy) -> [f32; 3] {
    let average = (luminous_efficacy.high_watt_factor - luminous_efficacy.low_watt_factor) * 0.5
        + luminous_efficacy.low_watt_factor;

    let low = watts / luminous_efficacy.low_watt_factor;
    let high = watts / luminous_efficacy.high_watt_factor;
    let avg = watts / average;

    [low, avg, high]
}

//todo write test.