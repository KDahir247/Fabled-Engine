// Responsible for converting ISO arithmetic speed to logarithmic speed and vis
// versa.

// Conversion form arithmetic speed S to logarithmic Speed so and rounding to
// the nearest integer.
// So = 10LogS + 1
pub fn arithmetic_to_logarithmic_speed(arithmetic_speed: f32) -> f32 {
    10.0 * arithmetic_speed.log10() + 1.0
}

// Conversion from logarithmic speed to arithmetic speed
// S = 10^((So - 1)/ 10)
pub fn logarithmic_to_arithmetic_speed(logarithmic_speed: f32) -> f32 {
    10.0f32.powf((logarithmic_speed - 1.0) / 10.0)
}

#[cfg(test)]
mod iso_speed_test {
    use crate::camera::{arithmetic_to_logarithmic_speed, logarithmic_to_arithmetic_speed};

    #[test]
    fn arithmetic_to_logarithmic() {
        let arithmetics = [
            0.8, 1.0, 1.2, 1.6, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 20.0, 25.0,
            32.0, 40.0, 50.0, 64.0, 80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 320.0, 400.0, 500.0,
            640.0, 800.0, 1000.0, 1250.0, 1600.0, 2000.0, 2500.0, 3200.0,
        ];

        let logarithmics = [
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0,
            30.0, 31.0, 32.0, 33.0, 34.0, 35.0, 36.0,
        ];

        for index in 0..arithmetics.len() {
            let logarithmic = arithmetic_to_logarithmic_speed(arithmetics[index]);
            assert!(logarithmic.round().eq(&logarithmics[index]))
        }
    }


    #[test]
    fn logarithmic_arithmetic() {
        let error_threshold = 20.0;

        let arithmetics = [
            1.0, 1.2, 1.6, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 20.0, 25.0, 32.0,
            40.0, 50.0, 64.0, 80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 320.0, 400.0, 500.0, 640.0,
            800.0, 1000.0, 1250.0, 1600.0, 2000.0, 2500.0, 3200.0,
        ];

        let logarithmics = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0, 33.0, 34.0, 35.0, 36.0,
        ];

        for index in 0..logarithmics.len() {
            let arithmetic = logarithmic_to_arithmetic_speed(logarithmics[index]);

            println!("{} {}", arithmetic, arithmetics[index]);
            assert!((arithmetic - arithmetics[index]) < error_threshold);
        }
    }
}
