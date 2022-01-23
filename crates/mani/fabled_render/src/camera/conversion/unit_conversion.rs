const RCP_1000: f32 = 1.0 / 1000.0_f32;
const RCP_39: f32 = 1.0 / 39.37_f32;
const RCP_25: f32 = 1.0 / 25.4;

pub fn millimeter_to_meters(millimeter: f32) -> f32 {
    millimeter * RCP_1000
}

pub fn meter_to_millimeter(meter: f32) -> f32 {
    meter * 1000.0
}


pub fn inch_to_meter(inch: f32) -> f32 {
    inch * RCP_39
}

pub fn meter_to_inch(meter: f32) -> f32 {
    meter * 39.37
}

pub fn inch_to_millimeter(inch: f32) -> f32 {
    inch * 25.4
}

pub fn millimeter_to_inch(millimeter: f32) -> f32 {
    millimeter / RCP_25
}


#[cfg(test)]
mod unit_conversion_test {
    use crate::camera::{
        inch_to_meter, inch_to_millimeter, meter_to_inch, meter_to_millimeter, millimeter_to_inch,
        millimeter_to_meters,
    };

    #[test]
    fn unit_conversion() {
        let meter_25 = millimeter_to_meters(25.0);
        assert!(meter_25.eq(&0.025));

        let millimeter_25 = meter_to_millimeter(meter_25);
        assert!(millimeter_25.eq(&25.0));

        let meter_123 = inch_to_meter(123.0);
        assert!(meter_123.eq(&3.1242063));

        let inch_123 = meter_to_inch(meter_123);
        assert!(inch_123.eq(&123.0));

        let millimeter_591 = inch_to_millimeter(591.0);
        assert!(millimeter_591.eq(&15011.399));

        let inch_591 = millimeter_to_inch(millimeter_591);
        assert!(inch_591.eq(&591.0));
    }
}
