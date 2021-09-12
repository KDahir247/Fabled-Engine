use crate::light::{ISOSpeed, CALIBRATION_CONSTANT_PRIMARY};

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

// For uniform, isotropic light source.
// Candela to lumens where apex angle is the full angle in degree of the the
// light apex.
// luminance intensity in candela (cd) to luminance flux in lumen (lm)
// Φv(lm) =  Iv(cd) × ( 2π(1 - cos(°/2)) )
pub fn candela_to_lumen(candela: f32, apex_angle: f32) -> f32 {
    let target_angle = apex_angle * std::f32::consts::PI / 180.0;
    candela * (std::f32::consts::TAU * (1.0 - (target_angle / 2.0).cos()))
}

// For uniform, isotropic light source,
// Opposite operation of candela to lumen
// Luminance flux in lumen (lm) to luminance intensity in candela (cd)
// cd = lm / ( 2π(1 - cos(º/2)) )
pub fn lumen_to_candela(lumen: f32, apex_angle: f32) -> f32 {
    let target_angle = apex_angle * std::f32::consts::PI / 180.0;
    lumen / (std::f32::consts::TAU * (1.0 - (target_angle / 2.0).cos()))
}


// EV100 to Luminance Chart
// where k is 12.5 and ISO speed is 100
//   _____________________
//   | EV100 | Luminance |
//   |-------|-----------|
//   | -4    | 0.008     |
//   | -3    | 0.016     |
//   | -2    | 0.031     |
//   | -1    | 0.063     |
//   | 0     | 0.125     |
//   | 1     | 0.25      |
//   | 2     | 0.5       |
//   | 3     | 1         |
//   | 4     | 2         |
//   | 5     | 4         |
//   | 6     | 8         |
//   | 7     | 16        |
//   | 8     | 32        |
//   | 9     | 64        |
//   | 10    | 128       |
//   | 11    | 256       |
//   | 12    | 512       |
//   | 13    | 1024      |
//   | 14    | 2048      |
//   | 15    | 4096      |
//   | 16    | 8192      |
//   ---------------------

// Convert EV (exposure value) to luminance (nit)
// where NIT is equivalent to Candela (cd/m^22), thus 1 Nit  = 1 cd/m^2
// NIT is a metric for measuring luminance, which is how much light an object
// emits
// Luminance = (2^EV * reflected-light meter calibration constant) / S
pub fn ev_to_luminance(ev: f32, iso: ISOSpeed, calibration_constant: Option<f32>) -> f32 {
    let calibration_constant = calibration_constant.unwrap_or(CALIBRATION_CONSTANT_PRIMARY);

    (2.0f32.powf(ev) * calibration_constant) / iso.arithmetic_speed
}

// Opposite operation of ev to luminance (nit)
// EV = log2((luminance * S) / reflected-light meter calibration constant)
pub fn luminance_to_ev(luminance: f32, iso: ISOSpeed, calibration_constant: Option<f32>) -> f32 {
    let calibration_constant = calibration_constant.unwrap_or(CALIBRATION_CONSTANT_PRIMARY);

    f32::log2((luminance * iso.arithmetic_speed) / calibration_constant)
}


// Calculate point light's luminance power (luminance flux) from the luminance
// intensity
// φ = 4π I
// which is equivalent to φ = 2τ I
// where τ = tau == 2π
// where φ = luminous power and I = luminous intensity
pub fn point_light_candela_to_lumen(candela: f32) -> f32 {
    2.0 * std::f32::consts::TAU * candela
}

// Opposite operation of point light luminance power (luminance flux) to the
// luminance Calculate point light's luminance intensity from luminance power
// (luminance flux)
// 	I = 4π / φ
// which is equivalent to I = φ / 2τ
// where τ = tau == 2π
pub fn point_light_lumen_to_candela(lumen: f32) -> f32 {
    lumen / (2.0 * std::f32::consts::TAU)
}

pub fn spot_light_candela_to_lumen(candela: f32) -> f32 {
    unimplemented!()
}

pub fn spot_light_lumen_to_candela(lumen: f32) -> f32 {
    unimplemented!()
}

/// result have been retrieve from [light calculation](https://www.rapidtables.com/calc/light/index.html)
#[cfg(test)]
mod unit_conversion_tests {
    use crate::light::{
        candela_to_lumen, candela_to_lux, ev_to_luminance, lumen_to_candela, luminance_to_ev,
        lux_to_candela, point_light_candela_to_lumen, point_light_lumen_to_candela, ISOSpeed,
    };

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

    #[test]
    fn ev_luminance() {
        // See EV100 to Luminance Chart above
        let ev = 7.0;
        let iso_arithmetic = ISOSpeed::default();
        let luminance = ev_to_luminance(ev, iso_arithmetic, None);
        assert!(luminance.eq(&16.0));

        let in_question_ev = luminance_to_ev(luminance, iso_arithmetic, None);
        assert!(in_question_ev.eq(&ev));
    }


    #[test]
    fn point_light_candela_lumen() {
        let candela = 150.0;

        let lumen = point_light_candela_to_lumen(candela);
        println!("{}", lumen);


        let in_question_candela = point_light_lumen_to_candela(lumen);
        println!("{}", in_question_candela);
    }
}
