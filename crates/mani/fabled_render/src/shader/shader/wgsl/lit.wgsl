//---------------------------- Vertex Shader ----------------------------

[[block]]
struct Uniforms{
    u_view_position : vec4<f32>;
    view : mat4x4<f32>;
    proj : mat4x4<f32>;
    inv_proj : mat4x4<f32>;

};

[[group(1), binding(0)]]
var<uniform> uniform : Uniforms;

struct VertexInput{
    [[location(0)]] position : vec3<f32>;
    [[location(1)]] tex_coord : vec2<f32>;
    [[location(2)]] normal : vec3<f32>;
    [[location(3)]] tangent : vec4<f32>;
};


struct VertexOutput{
    [[builtin(position)]] v_position : vec4<f32>;
    [[location(0)]] tex_coord : vec2<f32>;
    [[location(1)]] world_normal : vec3<f32>;
    [[location(2)]] world_position : vec3<f32>;
    [[location(3)]] view_dir : vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(in : VertexInput) -> VertexOutput{

    var out : VertexOutput;
    out.tex_coord = in.tex_coord;

    out.world_normal = in.normal;
    out.world_position = in.position;

    out.v_position = uniform.proj * uniform.view * vec4<f32>(in.position,1.0);
    out.view_dir = in.position - uniform.u_view_position.xyz;
    return out;
}

//---------------------------- Fragment Shader ----------------------------

[[block]]
struct MaterialColor {
    ambient  : vec4<f32>;
    diffuse  : vec4<f32>;
    specular : vec4<f32>;

    factor : vec3<f32>;

};

[[group(0), binding(0)]]
var<uniform> material_color : MaterialColor;

[[block]]
struct Light{
    position : vec3<f32>;
    ambient_color : vec4<f32>;
    diffuse_color : vec4<f32>;
    specular_color : vec4<f32>;
};

[[group(2), binding(0)]]
var<uniform> light : Light;


[[group(0), binding(1)]] var t_diffuse : texture_2d<f32>;
[[group(0), binding(2)]] var t_sampler : sampler;


fn calculate_diffuse_reflection(normal : vec3<f32>, light_dir : vec3<f32>, albedo : f32) -> vec3<f32>{
  let PI = 3.14159265358979323846264338327950288;

  // light color * saturation(normal, light_dir);
  let direct_color = light.diffuse_color.xyz * min(max(dot(normalize(normal), normalize(light_dir)), 0.0),1.0);

  let ambient_color = PI * material_color.ambient.xyz;

  // (Albedo [0f32, 1f32] * material.diffuse)
  let diffuse_color =  (albedo / PI) * material_color.diffuse.xyz;

  return ((ambient_color + direct_color) * diffuse_color);
}

fn calculate_specular_reflection(normal : vec3<f32>, half_way : vec3<f32>, alpha : f32, nl : f32) -> vec3<f32>{

    let highlight = pow(min(max(dot(normalize(normal), normalize(half_way)) ,0.0) ,1.0), alpha);

    return (light.specular_color.xyz * material_color.specular.xyz * highlight);
}

//todo
fn sample_environment_sample() -> vec4<f32>{

    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}

[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec4<f32>{

    let object_color : vec4<f32> = textureSample(t_diffuse, t_sampler, in.tex_coord);

    let light_dir = in.world_position - light.position;

    //diffuse
    let diffuse_reflection = calculate_diffuse_reflection(in.world_normal, light_dir, 0.5);

    //specular
    let half_way = vec3<f32>(1.0) + in.view_dir;
    let specular_reflection = calculate_specular_reflection(in.world_normal,half_way, 20.0, 0.0);

    let result = (diffuse_reflection + specular_reflection);

    // Dont need gamma correction since using sRGB buffer
    // let gamma : f32 = 2.0; //Range for  2.0 to 2.5
    // let corrected_result = pow(result.rgb,vec3<f32>(1.0 / gamma));

    return vec4<f32>(result * object_color.xyz, material_color.factor.y);
}
