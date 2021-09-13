// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.


// This can be for point or spot light. If it is for spot light then include get_angle_att function.

[[block]]
struct ShadowLightInfo{
    world_to_light : matrix4<f32>
    light_index : f32
};

[[group(1), binding(0)]]
var<uniform> light : ShadowLightInfo;

fn get_ies_profile_attenuation(norm_light_vector : vec3<f32>, light : ShadowLightInfo) -> f32{

    // Sample direction into light space.
    let ies_sample_direction : vec3<f32> = -norm_light_vector * light.world_to_light;

    // Cartesian to spherical space
    // Texture encoded with cos(phi), scale from -1->1 to 0->1

    let fabled_inv_two_pi : f32 = 0.15915494309189533576888376337251;

    let phi_coord : f32 = (ies_sample_direction.z * 0.5) + 0.5;
    let theta : f32 = atan2(ies_sample_direction.y, ies_sample_direction.x);
    let theta_coord = theta * fabled_inv_two_pi;
    let tex_coord : vec3<f32> = vec3<f32>(theta_coord, phi_coord, light.light_index);

    //should be the same as hlsl Texture.SampleLevel. We specify the mip level to 0.
    let ies_profile_scale : f32 = textureSampleLevel(ies_texture, sampler, tex_coord, 0).r;
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

    ........ calculating light and model.


    let un_normalized_light_vector : vec3<f32> = light_pos - world_pos;
    let norm_light_vector : vec3<f32> = normalize(un_normalized_light_vector);

    // Default value for sport light attenuation
    let attenuation : f32 = 1.0;


    attenuation *= get_angle_att(norm_light_vector, light_forward, light_angle_scale, light_angle_offset);
    attenuation *= get_ies_profile_attenuation(norm_light_vector, light);

    // light_color depends on options.
    // Non masked : light_color = color * max_candelas.
    // masked (for point light with luminance power) : light_color = color * phi / (4 * pi)

    let luminance : vec3<f32> = BSDF(...) * max(0.0, min(1.0, dot(light_norm_vector, norm_light_vector))) * light_color * attenuation;
}