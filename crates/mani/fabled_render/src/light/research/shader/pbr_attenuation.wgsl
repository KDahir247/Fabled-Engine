// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.

// This is for Spot Light


// saturate(1.0 - x^n / lightRadius^n)^2

// squared Distance = Distance * Distance
// invSqrAttRadius = 1.0 / (LightRadius * LightRadius)
fn smooth_distance_att(squared_distance : f32, inv_sqr_att_radius : f32) -> f32{

    // Distance^2 / LightRadius^2
    let factor : f32 = squared_distance * inv_sqr_att_radius;

    // saturate = max(0, min(1, VALUE))
    // equalivent to: saturate(1.0 - factor * factor)
    // 1.0 - ((Distance^2 / LightRadius^2) * (Distance^2 / LightRadius^2))
    let smooth_factor : f32 = max(0.0, min(1.0, 1.0 - factor * factor))

    return smooth_factor * smooth_factor
}

// Calculate the squared magnitude on the light vector and passes both inv_sqr_att_radius
// and sqrt_distance to smooth_distance_att and return smooth_distance_att result

// return smooth_distance_att((x * x + y * y + z * z), inv_sqr_att_radius)

fn get_distance_att(un_normalized_light_vector : vec3<f32>, inv_sqr_att_radius : f32) -> f32{

    // Get the squared magnitude of the vector
    // x * x + y * y + z * z
    let sqrt_distance : f32 = dot(un_normalized_light_vector, un_normalized_light_vector);

    let attenuation : f32 = smooth_distance_att(sqrt_distance, inv_sqr_att_radius);

    return attenuation;
}


fn get_angle_att(normalized_light_vector : vec3<f32>,
                  light_dir : vec3<f32>,
                  light_angle_scale : f32,
                  light_angle_offset : f32) -> f32{

                  // On the CPU side
                  // let light_angle_scale = 1.0f32 / std::f32::max(0.001f32, (cos_inner - cos_outer)));
                  // let light_angle_offset = -cos_outer * angle_scale;

                  let cd : f32 = dot(light_dir, normalized_light_vector);
                  var attenuation : f32 = max(0, min(1, cd * light_angle_scale + light_angle_offset));

                  // smooth the transition
                  attenuation *= attenuation;

                  return attenuation;
}



[[stage(fragment)]]
fn frag_main() -> [[location(0)]] vec4<f32>{

    ........ calculating spot light and model.



    let un_normalized_light_vector : vec3<f32> = light_pos - world_pos;
    let norm_light_vector : vec3<f32> = normalize(un_normalized_light_vector);

    // Default value for sport light attenuation
    let attenuation : f32 = 1.0;

    attenuation *= get_distance_att(un_normalized_light_vector, inv_sqr_att_radius);
    attenuation *= get_angle_att(norm_light_vector, light_forward, light_angle_scale, light_angle_offset);

    // light_color is the outgoing luminance of the light time the user light color
    // i .e with point light and luminance power unit: light_color = color * phi / (4.0 * PI)


    let luminance = BSDF(...) * max(0.0, min(1.0, dot(norm_light_vector, light_norm_vector))) * light_color * attenuation;


    ...... Continues here
}
