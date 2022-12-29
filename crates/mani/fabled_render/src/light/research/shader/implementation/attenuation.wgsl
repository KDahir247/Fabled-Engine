// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.


fn distance_attenuation(light_id : u32, world_position : vec3<f32>, light_radius : f32) -> f32{
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
    return saturate(pow((1.0 - pow(light_distance/light_radius, 4.0)),2.0)) / bakery;

    #else UnFiltered
    return saturate(pow((1.0 - pow(light_distance/light_radius, 4.0)),2.0)) / unfiltered;


    #else
    return saturate(pow((1.0 - pow(light_distance/light_radius, 4.0)),2.0)) / custom;


    #endif

}
