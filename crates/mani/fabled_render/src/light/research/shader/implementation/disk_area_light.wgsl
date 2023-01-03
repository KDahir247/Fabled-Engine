// Psedo code for wgsl


struct Common{
    normal : vec3<f32>, //normalized normal, in world space
    view : vec3<f32>, //normalized vector from the fragment to the eye
    // glsl reflect
    //reflected = reflect(-view, shading_normal)
    reflected : vec3<f32>, // reflection of view about normal
}

struct SunLight{
    illuminance_lux : f32, // intensity of light illuminance
    disk_radius : f32, // Disk radius
    distance_to_disk : f32, // Distance to disk
}

fn sample_sunlight(light_direction : vec3<f32>) -> vec3<f32>{

    let DdotR = dot(light_direction, reflected);

    let s = reflected - DdotR * light_direction;

    let l = vec3(0.0);

    if (DdotR < SunLight.distance_to_disk){
        l = normalize(SunLight.distance_to_disk * light_direction + normalize(s) * SunLight.disk_radius);
    }else{
        l = reflected;
    }

}

@fragment
fn fs_main(....) -> ....{

    let

    let illuminance = SunLight.illuminance_lux * saturate(dot(Common.normal, light_direction));

    // BSDF function.........


    BSDF * illuminance
}