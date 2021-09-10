// Candela to lux where the distance is in meters
// Luminous intensity in candela (cd) to illuminance in lux (lx)
// Ev(lx) = Iv(cd) / (d(m))^2
pub fn candela_to_lux(candela: f32, distance: f32) -> f32 {
    candela / (distance * distance)
}

// Opposite operation of candela to lux
// Illuminance in lux (lx) to Luminance intensity in candela (cd)
// Iv(cd) = Ev(lx) * (d(m))^2
pub fn lux_to_candela(lux: f32, distance: f32) -> f32 {
    lux * distance * distance
}

// Candela to lumens where apex angle is the full angle in degree of the the
// light apex.
// luminance intensity in candela (cd) to luminance flux in lumen (lm)
// Φv(lm) =  Iv(cd) × ( 2π(1 - cos(°/2)) )
pub fn candela_to_lumen(candela: f32, apex_angle: f32) -> f32 {
    let target_angle = apex_angle * std::f32::consts::PI / 180.0;
    candela * (std::f32::consts::TAU * (1.0 - (target_angle / 2.0).cos()))
}

// Opposite operation of candela to lumen
// Luminance flux in lumen (lm) to luminance intensity in candela (cd)
// cd = lm / ( 2π(1 - cos(º/2)) )
pub fn lumen_to_candela(lumens: f32, apex_angle: f32) -> f32 {
    let target_angle = apex_angle * std::f32::consts::PI / 180.0;
    lumens / (std::f32::consts::TAU * (1.0 - (target_angle / 2.0).cos()))
}

/// result have been retrieve from [light calculation](https://www.rapidtables.com/calc/light/index.html)
#[cfg(test)]
mod unit_conversion_tests {
    use crate::light::{candela_to_lumen, candela_to_lux, lumen_to_candela, lux_to_candela};

    #[test]
    fn candela_lux_test() {
        let error_threshold = 0.001;

        let luminance_intensity = 1000.0;
        let distance_meter = 15.5;

        let result = 4.16233053;

        let in_question = candela_to_lux(luminance_intensity, distance_meter);

        assert!((in_question - result).abs() < error_threshold);

        let revert_in_question = lux_to_candela(in_question, distance_meter);

        assert!(revert_in_question.eq(&luminance_intensity));
    }


    #[test]
    fn candela_lumens_test() {
        let error_threshold = 0.001;
        let luminance_intensity = 1234.0;
        // Flood light angle 32 to 45 degree angle
        // middle light angle of 32 and 45
        let apex_angle = 38.5;

        let result = 433.503_02;

        let in_question = candela_to_lumen(luminance_intensity, apex_angle);

        assert!((in_question - result).abs() < error_threshold);

        let revert_in_question = lumen_to_candela(in_question, apex_angle);

        assert!(revert_in_question.eq(&luminance_intensity));
    }
}
