// pseudo wgsl (WEBGPU) shader code. Don't use this as the actual implementation because it will be filled with errors.
// Just for research.

// The sun is handled as a disc area light always perpendicular to the outer hemisphere

[[block]]
struct SunLight{
    direction : vec3<f32>,
    angular_radius : f32
};

// There is usually only one sun so we will keep it as one light source.
[[group(0), binding(0)]]
var<uniform> sun : SunLight;


[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32>{

    ........ calculating light and model.

    let D : vec3<f32> = sun.direction;
    let disk_radius : f32 = sin(sun.angular_radius); // Disk radius
    let disk_distance : f32 = cos(sun.angular_radius); // Distance to disk


    // Closest point to a disk (since the radius is small, this is a good approx)
    let dir_dot_r : f32 = dot(D, R);
    let S : vec3<f32> = R - dir_dot_r * D;

    var L : vec3<f32>;

    if dir_dot_r < disk_distance{

        L = normalize(disk_distance * D + normalize(S) * disk_radius)

    }else{
        L = R;
    }

    // Diffuse and specular evaluation.
    // user defines the sunIlluminanceLux.
    let illuminance : f32 = sunIlluminanceLux * max(0.0, min(1.0, dot(light_normal_vector, D)));


    // D : diffuse direction use for diffuse lighting.
    // L : specular direction use with specular lighting.
    let luminance : f32 = BSDF(V, D, L, data) * illuminance;

}