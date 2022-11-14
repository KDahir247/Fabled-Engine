pub const CALIBRATION_CONSTANT_PRIMARY: f32 = 12.5;

pub const CALIBRATION_CONSTANT_SECONDARY: f32 = 14.0;

// todo got to look if it include standard f-stops
pub const ISO_ARITHMETIC_STANDARD: [f32; 37] = [
    0.8, 1.0, 1.2, 1.6, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 20.0, 25.0, 32.0,
    40.0, 50.0, 64.0, 80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 320.0, 400.0, 500.0, 640.0, 800.0,
    1000.0, 1250.0, 1600.0, 2000.0, 2500.0, 3200.0,
];

pub const SHUTTER_SPEED_STANDARD: [f32; 11] = [
    0.25,                               // 1/4
    0.125,                              // 1/8
    0.06666666666666666666666666666667, // 1/15
    0.03333333333333333333333333333333, // 1/30
    0.01666666666666666666666666666667, // 1/60
    0.008,                              // 1/125
    0.004,                              // 1/250
    0.002,                              // 1/500
    0.001,                              // 1/1000
    0.0005,                             // 1/2000
    0.00025,                            // 1/4000
];


// 0.6^11
pub const LENS_VIGNETTING_ATTENUATION: f32 = 0.65;


pub const SENSOR_SIZE: f32 = 0.024;
