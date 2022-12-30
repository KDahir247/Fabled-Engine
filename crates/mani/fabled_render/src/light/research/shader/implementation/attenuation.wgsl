// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.

//  increasing the radius_influence if accuracy is needed
fn distance_attenuation(light_id : u32, world_position : vec3<f32>, radius_influence : f32) -> f32{
    //Punctional light excluding directional light, since there is no attentuation applyed for it.
    let light_source = light[light_id];

    let light_vec = (light_source.position.xyz - world_position) * light_source.position.w;

    let light_distance = length(light_vec);
    let light_distance_sqr = lengths_qr(light_vec);


    let bakery = 1.0 / (light_distance_sqr + 1.0);

    let unfiltered = 1.0 / max(light_distance_sqr, 0.01 * 0.01);

    // where x is the custom light size specified.
    let custom = 1.0 / (light_distance_sqr + x * x);



    #if Bakery
    return saturate(pow((1.0 - pow(light_distance/radius_influence, 4.0) ),2.0)) / bakery;

    #else UnFiltered
    return saturate(pow((1.0 - pow(light_distance/radius_influence, 4.0)),2.0)) /   unfiltered;


    #else
    return saturate(pow((1.0 - pow(light_distance/radius_influence, 4.0) ),2.0)) /  custom;


    #endif

}


fn spotangle_attenuation(light_id : u32) -> f32{
    // Assuming that light_source is a spot light
    let light_source = light[light_id];

    let l = normalize(light_source.position);

    let light_dir = light_source.dir;

    // Calculated in the cpu side
    // let cos_outer = f32::cos(self.outer_cone);
    // 1.0 / f32::max(f32::cos(self.inner_cone) - cos_outer, 0.0001)
    let spot_scale = lightsource.spot_scale;
    // calculated in the cpu side
    // let cos_outer = f32::cos(self.outer_cone);
    // -cos_outer  * self.spot_scale()
    let spot_offset = lights_source.spot_offset;

    let cd = dot(normalize(-light_dir), l);
    let attenuation = saturate(cd * spot_scale + spot_offset);

    return attenuation * attenuation;
}

fn photometric_attenuation(light_id : u32, world_position : vec3<f32>, light_dir : vec3<f32>) -> f32 {
    let light_source = light[light_id];

    let light_vec = (light_source.position.xyz - world_position) * light_source.position.w;

    let cos_theta = dot(-light_vec, light_dir);
    let angle = acos(cos_theta) * (1.0 / PI);

    return textureLoad(ligh_profile_map, vec2(angle, 0.0), 0.0).r;

}


@fragment
fn fs_main(....) -> .....{

    // Do calculation.....

    // These are random numbers.
    let radius_influence = ....;

    let l = normalize(light_pos);
    let NoL = clamp(dot(n, l), 0.0, 1.0);

    let attenuation = distance_attenuation(1, frag_position, radius_influence);

    let luminance = (BSDF(....) * light_intensity * attenuation * NoL) * light_color;

    return luminance;
}
