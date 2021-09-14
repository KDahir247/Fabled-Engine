// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.


// Without Correct Horizon
[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32>{

    ........ calculating light and model.

    let world_normal : vec3<f32> = world_normal;
    let l_un_normalized : vec3<f32> = light_pos - world_pos;
    let l_normalized : vec3<f32> = normalize(l_un_normalized);
    let sqr_distance : f32 = dot(l_un_normalized, l_un_normalized);


    // Patch to Sphere frontal equation.
    let sqr_light_radius : f32  = light.radius * light.radius;
    // Do not allow object to penetrate the light (max)
    // Form factor equation include a (1 / FABLED_PI) that need to be cancel
    // thus the "FABLED_PI *"
    let illuminance : f32 = FABLED_PI * (sqr_light_radius / (max(sqr_light_radius, sqr_distance))) * max(0.0, min(1.0, dot(world_normal, l_normalized)));
}




// With Correct Horizon
[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32>{

    ........ calculating light and model.

    let world_normal : vec3<f32> = world_normal;
    let l_un_normalized : vec3<f32> = light_pos - world_pos;
    let l_normalized : vec3<f32> = normalize(l_un_normalized);
    let sqr_distance : f32 = dot(l_un_normalized, l_un_normalized);

    // Tilted patch to sphere equation.
    let beta : f32 = acos(dot(world_normal, l_normalized));
    let H : f32 = sqrt(sqr_distance);
    let h : f32 = H / radius;
    let x : f32 = sqrt(h * h - 1.0);
    let y : f32 = -x * (1.0 / tan(beta));


    var illuminance : f32;

    if (h * cos(beta) > 1.0){
        illuminance = cos(beta) / (h * h);
    }else{
        illuminance = (1.0 / (FABLED_PI * h * h)) *
              (cos(beta) * acos(y) - x * sin(beta) * sqrt(1.0 - y * y)) +
              (1.0 / FABLED_PI) * atan(sin(beta) * sqrt(1.0 - y * y) / x);
    }

    illuminance *= FABLED_PI;
}