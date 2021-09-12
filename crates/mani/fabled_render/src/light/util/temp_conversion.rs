// Utility for Light Appearance Temperature

// Convert Kelvin to Celsius for temperature measurement T(°)
// T(°C) = T(K) - 273.15
pub fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - 273.15
}

// Convert Celsius to Kelvin for temperature measurement T(°)
// T(K) = T(°C) + 273.15
pub fn celsius_to_kelvin(celsius: f32) -> f32 {
    celsius + 273.15
}

// Convert Kelvin to Fahrenheit for temperature measurement T(°)
// T (°F) = T(K) × 9/5 - 459.67
pub fn kelvin_to_fahrenheit(kelvin: f32) -> f32 {
    // 1.8 == 9.0 / 5.0
    kelvin * 1.8 - 459.67
}

// Convert Fahrenheit to Kelvin for temperature measurement T(°)
// T(K) = (T(°F) + 459.67)× 5/9
pub fn fahrenheit_to_kelvin(fahrenheit: f32) -> f32 {
    // 0.555... == 5.0 / 9.0
    (fahrenheit + 459.67) * 0.555_555_6
}

// Convert Celsius to Fahrenheit for temperature measurement T(°)
// T(°F) = T(°C) × 9/5 + 32 which is equivalent T(°F) = T(°C) × 1.8 + 32
pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}


// Convert Fahrenheit to Celsius for temperature measurement T(°)
// T(°C) = (T(°F) - 32) / (9/5) which is equivalent T(°C) = (T(°F) - 32) / 1.8
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

#[cfg(test)]
mod temperature_test {
    use crate::light::{
        celsius_to_fahrenheit, celsius_to_kelvin, fahrenheit_to_celsius, fahrenheit_to_kelvin,
        kelvin_to_celsius, kelvin_to_fahrenheit,
    };

    #[test]
    fn kelvin_celsius() {
        let target_kelvin = 6500.0;
        let celsius = kelvin_to_celsius(target_kelvin);
        println!("{}", celsius);

        assert!(celsius.eq(&6226.85));

        let in_question_kelvin = celsius_to_kelvin(celsius);
        println!("{}", in_question_kelvin);

        assert!(in_question_kelvin.eq(&target_kelvin));
    }

    #[test]
    fn kelvin_fahrenheit() {
        let error_threshold = 0.001;

        let target_kelvin = 7_348.823;
        let fahrenheit = kelvin_to_fahrenheit(target_kelvin);
        println!("{}", fahrenheit);

        assert!((fahrenheit - 12_768.211).abs() < error_threshold);

        let in_question_kelvin = fahrenheit_to_kelvin(fahrenheit);
        println!("{}", in_question_kelvin);

        assert!((target_kelvin - in_question_kelvin).abs() < error_threshold);
    }

    #[test]
    fn celsius_fahrenheit() {
        let error_threshold = 0.001;

        let extremely_hot_weather = 45.3;

        let target_fahrenheit = 113.54;
        let fahrenheit = celsius_to_fahrenheit(extremely_hot_weather);
        println!("{}", fahrenheit);


        assert!((fahrenheit - target_fahrenheit).abs() < error_threshold);

        let in_question_celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}", in_question_celsius);

        assert!((extremely_hot_weather - in_question_celsius).abs() < error_threshold);
    }
}
