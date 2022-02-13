#[derive(Copy, Clone, Debug)]
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


pub fn watt_to_lumen(watts: f32, luminous_efficacy: Efficacy) -> [f32; 3] {
    let average = (luminous_efficacy.high_lumen_per_watt - luminous_efficacy.low_lumen_per_watt)
        * 0.5
        + luminous_efficacy.low_lumen_per_watt;

    let low = watts * luminous_efficacy.low_lumen_per_watt;
    let high = watts * luminous_efficacy.high_lumen_per_watt;
    let avg = watts * average;

    [low, avg, high]
}

pub fn lumen_to_watt(lumen: f32, luminous_efficacy: Efficacy) -> [f32; 3] {
    let average = (luminous_efficacy.high_lumen_per_watt - luminous_efficacy.low_lumen_per_watt)
        * 0.5
        + luminous_efficacy.low_lumen_per_watt;


    let low = lumen / luminous_efficacy.low_lumen_per_watt;
    let high = lumen / luminous_efficacy.high_lumen_per_watt;
    let avg = lumen / average;

    [low, avg, high]
}

#[cfg(test)]
mod efficacy_test {
    use crate::light::{lumen_to_watt, watt_to_lumen, Efficacy};

    #[test]
    fn watt_lumen_test() {
        // Test in
        // https://www.thecalculatorsite.com/energy/watts-lumens.php

        const INCANDESCENT_RESULT: [f32; 3] = [480.0, 600.0, 720.0];


        let incandescent_efficacy = Efficacy::INCANDESCENT;
        let [lumen_low, lumen_average, lumen_high] = watt_to_lumen(40.0, incandescent_efficacy);

        assert_eq!((lumen_low - INCANDESCENT_RESULT[0]), 0.0);
        assert_eq!((lumen_average - INCANDESCENT_RESULT[1]), 0.0);
        assert_eq!((lumen_high - INCANDESCENT_RESULT[2]), 0.0);


        const CFL_RESULT: [f32; 3] = [4960.0, 6200.0, 7440.0];

        let cfl_efficacy = Efficacy::CFL;

        let [lumen_low, lumen_average, lumen_high] = watt_to_lumen(124.0, cfl_efficacy);

        assert_eq!((lumen_low - CFL_RESULT[0]), 0.0);
        assert_eq!((lumen_average - CFL_RESULT[1]), 0.0);
        assert_eq!((lumen_high - CFL_RESULT[2]), 0.0);
    }

    #[test]
    fn lumen_watt_test() {
        // Test in
        // https://www.thecalculatorsite.com/energy/lumens-watts.php

        const INCANDESCENT_RESULT: [f32; 3] = [42.0, 33.0, 28.0];

        let incandescent_efficacy = Efficacy::INCANDESCENT;
        let [watt_low, watt_avg, watt_high] = lumen_to_watt(500.0, incandescent_efficacy);

        assert_eq!((watt_low.round() - INCANDESCENT_RESULT[0]), 0.0);
        assert_eq!((watt_avg.round() - INCANDESCENT_RESULT[1]), 0.0);
        assert_eq!((watt_high.round() - INCANDESCENT_RESULT[2]), 0.0);

        const CFL_RESULT: [f32; 3] = [16.0, 13.0, 10.0];

        let cfl_efficacy = Efficacy::CFL;
        let [watt_low, watt_avg, watt_high] = lumen_to_watt(627.0, cfl_efficacy);


        assert_eq!((watt_low.round() - CFL_RESULT[0]), 0.0);
        assert_eq!((watt_avg.round() - CFL_RESULT[1]), 0.0);
        assert_eq!((watt_high.round() - CFL_RESULT[2]), 0.0);
    }
}
