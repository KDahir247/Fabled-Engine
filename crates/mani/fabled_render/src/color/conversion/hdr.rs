use fabled_math::FloatExtension;

pub fn screen_referred_to_rgbe(screen_referred_color: [f32; 3]) -> [f32; 4] {
    let [red, green, blue] = screen_referred_color;

    let maximum_luminance = red.max(green.max(blue));

    let scalar = f32::from(u8::from(maximum_luminance > 10.0f32.powf(-38.0)));

    let exponent = maximum_luminance.log2() + 128.0;

    let rgbe_intermediate = 2.0f32.powf(exponent - 128.0).recip();

    let red = (256.0 * red) * rgbe_intermediate;
    let green = (256.0 * green) * rgbe_intermediate;
    let blue = (256.0 * blue) * rgbe_intermediate;

    [
        red * scalar,
        green * scalar,
        blue * scalar,
        exponent * scalar,
    ]
}

pub fn rgbe_to_screen_referred(rgbe: [f32; 4]) -> [f32; 3] {
    let [red, green, blue, exponent] = rgbe;

    let maximum_luminance = red.max(green.max(blue));

    let scalar = f32::from(u8::from(maximum_luminance > 10.0f32.powf(-38.0)));

    let screen_referred_intermediate = 256.0 * 2.0f32.powf(exponent - 128.0);

    let red = (red + 0.5) * screen_referred_intermediate;
    let green = (green + 0.5) * screen_referred_intermediate;
    let blue = (blue + 0.5) * screen_referred_intermediate;


    [red * scalar, green * scalar, blue * scalar]
}


pub fn linear_to_rgb_m(linear: [f32; 3]) -> [f32; 4] {
    // linear to gamma
    let mut rgbm = [linear[0].sqrt(), linear[1].sqrt(), linear[2].sqrt(), 1.0];

    let range_rcp = 6.0f32.recip();

    let [red, green, blue, mut multiply] = [
        rgbm[0] * range_rcp,
        rgbm[1] * range_rcp,
        rgbm[2] * range_rcp,
        rgbm[3],
    ];

    let max_luminance = red.max(green.max(blue.max(1e-6))).clamp(range_rcp, 1.0);

    multiply = (max_luminance * 255.0).ceil() / 255.0;
    let rcp_multiply = multiply.recip();

    rgbm = [
        (red * rcp_multiply).saturate(),
        (green * rcp_multiply).saturate(),
        (blue * rcp_multiply).saturate(),
        multiply,
    ];

    rgbm
}

pub fn rgb_m_to_linear(rgbm: [f32; 4]) -> [f32; 3] {
    let [red, green, blue, multiple] = rgbm;
    let intermediate_step = 6.0 * multiple;

    // powi = gamma to linear (x * x)
    [
        (red * intermediate_step).powi(2),
        green * intermediate_step.powi(2),
        blue * intermediate_step.powi(2),
    ]
}
