 // ########### POISSON SHADOW SAMPLE #########

 var<private> poissonDisk : array<vec2<f32>, 64>= array<vec2<f32>,64>(
    vec2<f32>(-0.5119625, -0.4827938),
    vec2<f32>(-0.2171264, -0.4768726),
    vec2<f32>(-0.7552931, -0.2426507),
    vec2<f32>(-0.7136765, -0.4496614),
    vec2<f32>(-0.5938849, -0.6895654),
    vec2<f32>(-0.3148003, -0.7047654),
    vec2<f32>(-0.42215, -0.2024607),
    vec2<f32>(-0.9466816, -0.2014508),
    vec2<f32>(-0.8409063, -0.03465778),
    vec2<f32>(-0.6517572, -0.07476326),
    vec2<f32>(-0.1041822, -0.02521214),
    vec2<f32>(-0.3042712, -0.02195431),
    vec2<f32>(-0.5082307, 0.1079806),
    vec2<f32>(-0.08429877, -0.2316298),
    vec2<f32>(-0.9879128, 0.1113683f),
    vec2<f32>(-0.3859636, 0.3363545),
    vec2<f32>(-0.1925334, 0.1787288),
    vec2<f32>(0.003256182, 0.138135),
    vec2<f32>(-0.8706837, 0.3010679),
    vec2<f32>(-0.6982038, 0.1904326),
    vec2<f32>(0.1975043, 0.2221317),
    vec2<f32>(0.1507788, 0.4204168),
    vec2<f32>(0.3514056, 0.09865579),
    vec2<f32>(0.1558783, -0.08460935),
    vec2<f32>(-0.068497f, 0.4461993),
    vec2<f32>(0.3780522, 0.3478679),
    vec2<f32>(0.3956799, -0.1469177),
    vec2<f32>(0.5838975, 0.1054943),
    vec2<f32>(0.6155105, 0.3245716),
    vec2<f32>(0.3928624, -0.4417621),
    vec2<f32>(0.1749884, -0.4202175),
    vec2<f32>(0.6813727, -0.2424808),
    vec2<f32>(-0.6707711, 0.4912741),
    vec2<f32>(0.0005130528, -0.8058334),
    vec2<f32>(0.02703013, -0.6010728),
    vec2<f32>(-0.1658188, -0.9695674),
    vec2<f32>(0.4060591, -0.7100726),
    vec2<f32>(0.7713396, -0.4713659),
    vec2<f32>(0.573212, -0.51544),
    vec2<f32>(-0.3448896, -0.9046497),
    vec2<f32>(0.1268544, -0.9874692),
    vec2<f32>(0.7418533, -0.6667366),
    vec2<f32>(0.3492522, 0.5924662),
    vec2<f32>(0.5679897, 0.5343465),
    vec2<f32>(0.5663417, 0.7708698),
    vec2<f32>(0.7375497, 0.6691415),
    vec2<f32>(0.2271994, -0.6163502),
    vec2<f32>(0.2312844, 0.8725659),
    vec2<f32>(0.4216993, 0.9002838),
    vec2<f32>(0.4262091, -0.9013284),
    vec2<f32>(0.2001408, -0.808381),
    vec2<f32>(0.149394, 0.6650763),
    vec2<f32>(-0.09640376, 0.9843736),
    vec2<f32>(0.7682328, -0.07273844),
    vec2<f32>(0.04146584, 0.8313184),
    vec2<f32>(0.9705266, -0.1143304),
    vec2<f32>(0.9670017, 0.1293385),
    vec2<f32>(0.9015037, -0.3306949),
    vec2<f32>(-0.5085648, 0.7534177),
    vec2<f32>(0.9055501, 0.3758393),
    vec2<f32>(0.7599946, 0.1809109),
    vec2<f32>(-0.2483695, 0.7942952),
    vec2<f32>(-0.4241052, 0.5581087),
    vec2<f32>(-0.1020106, 0.6724468),
    );

fn interleaved_gradient_noise(uv : vec2<f32>) -> f32{
    return fract(52.9829189 * fract(dot(uv, vec2(0.06711056,0.00583715))));
}

fn rotate_poisson_disk(uv : vec2<f32>) -> mat2x2<f32>{
    let gradient = interleaved_gradient_noise(uv);

    let rotation_x = cos(gradient * 2.0 * 3.1415);
    let rotation_y = sin(gradient * 2.0 * 3.1415);

    return mat2x2(rotation_x, rotation_y, -rotation_y, rotation_x);
}
// #############################


// use as bias. fails in areas of high curvature. Dont use if scene has geometry with alot of curves. Use custom bias for high curvature.
fn retriever_depth_plane_bias(shadow_coord_ddx : vec3<f32>, shadow_coord_ddy : vec3<f32>,texel_size : vec2<f32>, bias_multiplier : f32) -> vec3<f32>{

    let bias_uv = vec2(
        shadow_coord_ddy.y * shadow_coord_ddx.z - shadow_coord_ddx.y * shadow_coord_ddy.z,
        shadow_coord_ddx.x * shadow_coord_ddy.z - shadow_coord_ddy.x * shadow_coord_ddx.z,
    );


    let bias_xy = bias_uv * bias_multiplier / ((shadow_coord_ddx.x * shadow_coord_ddy.y) - (shadow_coord_ddx.y * shadow_coord_ddy.x));

    let fractional_sample_error = dot(texel_size, abs(bias_xy));

    let bias_z = -min(fractional_sample_error, 0.001);

    return vec3<f32>(bias_xy.x, bias_xy.y, bias_z);
}


