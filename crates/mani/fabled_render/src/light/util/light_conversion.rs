use crate::camera::{ISOSpeed, CALIBRATION_CONSTANT_PRIMARY};

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

// From punctual light candela and luminance are the same, so we can just call
// ev_to_luminance
pub fn ev_to_candela(ev: f32, iso: ISOSpeed, calibration_constant: Option<f32>) -> f32 {
    ev_to_luminance(ev, iso, calibration_constant)
}

// Since luminance is the same as candela for punctual light, we can say that
// candela_to_ev is the same as luminance_to_ev1
pub fn candela_to_ev(candela: f32, iso: ISOSpeed, calibration_constant: Option<f32>) -> f32 {
    luminance_to_ev(candela, iso, calibration_constant)
}

// Since luminance is the same as candela for punctual light, we can convert the
// lux to candela, Then convert the candela which is equivalent to luminance for
// punctual light to ev pseudo code Convert Lux to Candela, then Convert Candela
// to Ev
pub fn lux_to_ev(lux: f32, distance: f32, iso: ISOSpeed, calibration_constant: Option<f32>) -> f32 {
    let candela = lux_to_candela(lux, distance);
    candela_to_ev(candela, iso, calibration_constant)
}


pub fn ev_to_lux(ev: f32, distance: f32, iso: ISOSpeed, calibration_constant: Option<f32>) -> f32 {
    let candela = ev_to_candela(ev, iso, calibration_constant);
    candela_to_lux(candela, distance)
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

// Opposite operation of point light luminance power (luminance flux) to
// luminance. Calculate point light's luminance intensity from luminance power
// (luminance flux)
// 	I = 4π / φ
// which is equivalent to I = φ / 2τ
// where τ = tau == 2π
pub fn point_light_lumen_to_candela(lumen: f32) -> f32 {
    lumen / (2.0 * std::f32::consts::TAU)
}


// Calculate spot light's luminance power (luminance flux) from the luminance
// intensity
// φ = 2π(1.0 - cos(θ outer / 2.0)) I
// which is equivalent to φ = τ(1.0 - cos(θ outer * 0.5)) I
// where τ = tau == 2π
pub fn spot_light_candela_to_lumen(candela: f32, outer_angle_rad: f32) -> f32 {
    std::f32::consts::TAU * (1.0 - (outer_angle_rad * 0.5)) * candela
}

// Opposite operation of spot light luminance power (luminance flux) to
// luminance. Calculate spot light's luminance intensity from luminance power
// (luminance flux)
// I = φ / 2π(1.0 - cos(θ outer / 2.0))
// which is equivalent to I = φ / τ(1.0 - cos(θ outer / 2.0))
// where τ = tau == 2π
pub fn spot_light_lumen_to_candela(lumen: f32, outer_angle_rad: f32) -> f32 {
    lumen / (std::f32::consts::TAU * (1.0 - (outer_angle_rad * 0.5)))
}

// Calculate approximate spot light's luminance power (luminance flux) from the
// luminance intensity (disregard outer_angle),
// φ = πI
pub fn spot_light_approx_candela_to_lumen(candela: f32) -> f32 {
    std::f32::consts::PI * candela
}


// Oposite operation of spot light luminance power (luminance flux) to
// luminance. Calculate approximate spot light's luminance intensity from
// luminance power (luminance flux) (disregarding outer_angle)
// I = φ / π
pub fn spot_light_approx_lumen_to_candela(lumen: f32) -> f32 {
    lumen / std::f32::consts::PI
}

// Calculate frustum light's luminance power (luminance flux) from
// luminance intensity
// φ = I * (4 * arcsin [ sin(θa / 2) * sin(θb / 2)])
// which is equivalent to φ = I * (4 * arcsin [ sin(θa * 0.5) * sin(θb * 0.5)])
pub fn frustum_light_candela_to_lumen(
    candela: f32,
    opening_angle_a_rad: f32,
    opening_angle_b_rad: f32,
) -> f32 {
    candela
        * (4.0
            * f32::asin(f32::sin(opening_angle_a_rad * 0.5) * f32::sin(opening_angle_b_rad * 0.5)))
}

// Opposite operation of frustum light's luminance power (luminance flux) to
// luminance. Calculate frustum light's luminance intensity from luminance power
// I = φ / (4 * arcsin [ sin(θa / 2) * sin(θb  / 2)] )
// which is equivalent to I = φ / (4 * arcsin [ sin(θa * 0.5) * sin(θb * 0.5)])
pub fn frustum_light_lumen_to_candela(
    lumen: f32,
    opening_angle_a_rad: f32,
    opening_angle_b_rad: f32,
) -> f32 {
    lumen
        / (4.0
            * f32::asin(f32::sin(opening_angle_a_rad * 0.5) * f32::sin(opening_angle_b_rad * 0.5)))
}

// Calculate Sphere area light's luminance(cd.m−2) from luminance power
// (luminance flux) L = φ / (4 * radius^2 * π^2)
pub fn sphere_area_light_lumen_to_luminance(lumen: f32, radius: f32) -> f32 {
    lumen / (4.0 * radius * radius * std::f32::consts::PI * std::f32::consts::PI)
}

// Opposite operation of sphere area light's luminance power (luminance flux) to
// luminance(cd.m−2). Calculate sphere area light's luminance power (luminance
// flux) from luminance(cd.m−2)
// φ = L * (4 * radius^2 * π^2)
pub fn sphere_area_light_luminance_to_lumen(luminance: f32, radius: f32) -> f32 {
    luminance * (4.0 * radius * radius * std::f32::consts::PI * std::f32::consts::PI)
}


// Calculate disk area light's luminance (cd.m−2) from luminance power
// (luminance flux)
// L = φ / (radius^2 * π^2)
pub fn disk_area_light_lumen_to_luminance(lumen: f32, radius: f32) -> f32 {
    lumen / (radius * radius * std::f32::consts::PI * std::f32::consts::PI)
}

// Opposite operation of disk area light's luminance(cd.m−2) from luminance
// power (luminance flux). Calculate disk area light's luminance power
// (luminance flux) from luminance
// φ = L * (radius^2 * π^2)
pub fn disk_area_light_luminance_to_lumen(luminance: f32, radius: f32) -> f32 {
    luminance * (radius * radius * std::f32::consts::PI * std::f32::consts::PI)
}

// Calculate tube area light's luminance (cd.m–2) from luminance power
// (luminance flux)
// L = φ / (2π * radius * width * 4π * radius^2) * π
pub fn tube_area_light_lumen_to_luminance(lumen: f32, width: f32, radius: f32) -> f32 {
    lumen
        / ((2.0 * std::f32::consts::PI * width + 4.0 * std::f32::consts::PI * radius * radius)
            * std::f32::consts::PI)
}

// Opposite operation of tube area light's luminance (cd.m–2) from luminance
// power (luminance flux). Calculate tube area light's luminance power
// (luminance flux) from luminance (cd.m–2)
// φ = L * (2π * radius * width * 4π * radius^2) * π
pub fn tube_area_light_luminance_to_lumen(luminance: f32, width: f32, radius: f32) -> f32 {
    luminance
        * ((2.0 * std::f32::consts::PI * width + 4.0 * std::f32::consts::PI * radius * radius)
            * std::f32::consts::PI)
}


// Calculate rectangle area light luminance (cd.m–2) from luminance power
// (luminance flux)
// L = φ / (width * height * π)
pub fn rectangle_area_light_lumen_to_luminance(lumen: f32, width: f32, height: f32) -> f32 {
    lumen / (width * height * std::f32::consts::PI)
}

// Opposite of rectangle area light luminance (cd.m–2) from luminance power
// (luminance flux) Calculate rectangle area light luminance power (luminance
// flux) from luminance.
// φ = L * (width * height * π)
pub fn rectangle_area_light_luminance_to_lumen(luminance: f32, width: f32, height: f32) -> f32 {
    luminance * (width * height * std::f32::consts::PI)
}

/// result have been retrieve from [light calculation](https://www.rapidtables.com/calc/light/index.html)
#[cfg(test)]
mod unit_conversion_tests {
    use crate::light::{
        candela_to_lumen, candela_to_lux, disk_area_light_lumen_to_luminance,
        disk_area_light_luminance_to_lumen, ev_to_luminance, frustum_light_candela_to_lumen,
        frustum_light_lumen_to_candela, lumen_to_candela, luminance_to_ev, lux_to_candela,
        point_light_candela_to_lumen, point_light_lumen_to_candela,
        rectangle_area_light_lumen_to_luminance, rectangle_area_light_luminance_to_lumen,
        sphere_area_light_lumen_to_luminance, sphere_area_light_luminance_to_lumen,
        spot_light_approx_candela_to_lumen, spot_light_approx_lumen_to_candela,
        spot_light_candela_to_lumen, spot_light_lumen_to_candela,
        tube_area_light_lumen_to_luminance, tube_area_light_luminance_to_lumen,
    };

    use crate::camera::ISOSpeed;

    #[test]
    fn candela_lux_test() {
        let error_threshold = 0.001;

        let luminance_intensity = 1000.0;
        let distance_meter = 15.5;

        let result = 4.162_330_6;

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
        assert!(in_question_candela.eq(&candela));
    }

    #[test]
    fn spot_light_candela_lumen() {
        let candela = 175.0;

        let spot_light_angle = 45.0f32;

        let lumen = spot_light_candela_to_lumen(candela, spot_light_angle.to_radians());
        println!("{}", lumen);


        let in_question_candela = spot_light_lumen_to_candela(lumen, spot_light_angle.to_radians());
        assert!(in_question_candela.eq(&candela));
    }


    #[test]
    fn spot_light_approx_candela_lumen() {
        let candela = 175.0;

        let lumen = spot_light_approx_candela_to_lumen(candela);
        println!("{}", lumen);

        let in_question_candela = spot_light_approx_lumen_to_candela(lumen);
        assert!(in_question_candela.eq(&candela));
    }

    #[test]
    fn frustum_light_candela_lumen() {
        let candela = 231.0;
        let angle_1 = 43.3f32.to_radians();
        let angle_2 = 15.123f32.to_radians();

        let lumen = frustum_light_candela_to_lumen(candela, angle_1, angle_2);
        println!("{}", lumen);

        let in_question_candela = frustum_light_lumen_to_candela(lumen, angle_1, angle_2);
        assert!(in_question_candela.eq(&candela));
    }

    #[test]
    fn sphere_area_light_lumen_luminance() {
        // a sphere area  light of radius 15 cm and emitting 1500 lm will have a
        // luminance of 1688 cd.m−2.

        let lumen = 1500.0;
        let radius_centimeter_to_meter = 15.0 / 100.0;

        let luminance = sphere_area_light_lumen_to_luminance(lumen, radius_centimeter_to_meter);
        let truncated_luminance = luminance.trunc();
        assert!(truncated_luminance.eq(&1688.0));

        let in_question_lumen =
            sphere_area_light_luminance_to_lumen(luminance, radius_centimeter_to_meter);
        assert!(in_question_lumen.eq(&lumen));
    }


    #[test]
    fn disk_area_light_lumen_luminance() {
        let lumen = 1500.0;
        let radius_centimeter_to_meter = 15.0 / 100.0;
        let luminance = disk_area_light_lumen_to_luminance(lumen, radius_centimeter_to_meter);
        let truncated_luminance = (luminance / 4.0).trunc();

        // Similar to sphere area light. so we can divide it by 4 to get the same result
        // as sphere area light
        println!("{}", truncated_luminance);
        assert!(truncated_luminance.eq(&1688.0));

        let in_question_lumen =
            disk_area_light_luminance_to_lumen(luminance, radius_centimeter_to_meter);
        println!("{}", in_question_lumen);

        assert!(in_question_lumen.eq(&lumen));
    }


    #[test]
    fn tube_area_light_lumen_luminance() {
        let lumen = 15000.0;
        let radius_centimeter_to_meter = 30.0 / 100.0;

        let luminance = tube_area_light_lumen_to_luminance(lumen, 15.0, radius_centimeter_to_meter);
        println!("{}", luminance);

        let in_question_lumen =
            tube_area_light_luminance_to_lumen(luminance, 15.0, radius_centimeter_to_meter);
        assert!(in_question_lumen.eq(&lumen));
    }

    #[test]
    fn rectangle_area_light_lumen_luminance() {
        let lumen = 15670.0;
        let width = 100.0;
        let height = 10.0;

        let luminance = rectangle_area_light_lumen_to_luminance(lumen, width, height);
        println!("{}", luminance);

        let in_question_lumen = rectangle_area_light_luminance_to_lumen(luminance, width, height);
        println!("{}", in_question_lumen);
        let rounded_in_question_lumen = in_question_lumen.round();
        println!("{}", rounded_in_question_lumen);
        assert!(rounded_in_question_lumen.eq(&lumen));
    }
}
