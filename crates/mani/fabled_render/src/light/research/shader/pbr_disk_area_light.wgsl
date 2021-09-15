// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.


fn cot(x : f32) -> f32{
    return cos(x) / sin(x);
}

fn acot(x : f32) -> f32{
    return atan(1 / x);
}

// Without Correct Horizon
[[stage(fragment)]]
fn fs_main() -> [[location(0)]]{

    ........ calculating spot light and model.


    let l_un_normalized : vec3<f32> = light_pos - world_pos;
    let l : vec3<f32> = normalize(l_un_normalized);


    // Form factor equation include a (1 / FABLED_PI) that need to be cancel
    // thus the "FABLED_PI *"
    let illuminance : f32 = FABLED_PI * max(0.0, min(1.0, dot(plane_normal, -l))) *
                            max(0.0, min(1.0, dot(world_normal, l))) /
                            (sqrt_distance / (radius * radius) + 1);


}



// With Correct horizon
fn fs_main() -> [[location(0)]]{

    ........ calculating spot light and model.

    let l_un_normalized : vec3<f32> = light_pos - world_pos;
    let l : vec3<f32> = normalize(l_un_normalized);

    //Nearly exact solution with horizon
    let h : f32 = length(light_pos - world_pos);
    let r : f32 = light_radius;
    let theta : f32 = acos(dot(world_normal, l));

    let H : f32 = h / r;
    let H2 : f32 = H * H;
    let X : f32 = pow((1 - H2 * cot(theta) * cot(theta)), 0.5);

    var illuminance : f32;

    if (theta < acot(1 / H)){
        illuminance = (1 / (1 / H2)) * cos(theta);
    }else{
        illuminance = -H * X * sin(theta) / (FABLED_PI * (1 + H2)) +
            (1 / FABLED_PI) * atan(X * sin(theta) / H) +
            cos(theta) * (FABLED_PI - acos(H * cot(theta))) / (FB_PI * (1 + H2));
    }

}


