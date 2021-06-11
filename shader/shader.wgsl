//---------------------------- Vertex Shader ----------------------------

[[block]]
struct Uniforms{
    u_view_position : vec3<f32>;
    view_proj : mat4x4<f32>;
};

[[group(1), binding(0)]]
var<uniform> uniform : Uniforms;


struct VertexInput{
    [[location(0)]] position : vec3<f32>;
    [[location(1)]] tex_coord : vec2<f32>;
    [[location(2)]] normal : vec3<f32>;
};


struct VertexOutput{
    [[builtin(position)]] v_position : vec4<f32>;
    [[location(0)]] tex_coord : vec2<f32>;
    [[location(1)]] world_normal : vec3<f32>;
    [[location(2)]] world_position : vec3<f32>;
};


[[stage(vertex)]]
fn vs_main(in : VertexInput) -> VertexOutput{

    var out : VertexOutput;
    out.tex_coord = in.tex_coord;

    out.world_normal = in.normal;
    out.world_position = in.position;

    out.v_position = uniform.view_proj * vec4<f32>(in.position,1.0);
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
    direction : vec3<f32>;
    ambient_color : vec4<f32>;
    diffuse_color : vec4<f32>;
    specular_color : vec4<f32>;
};

[[group(2), binding(0)]]
var<uniform> light : Light;


[[group(0), binding(1)]] var t_diffuse : texture_2d<f32>;
[[group(0), binding(2)]] var t_sampler : sampler;

[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec4<f32>{

    let object_color : vec4<f32> = textureSample(t_diffuse, t_sampler, in.tex_coord);

    //ambient
    let ambient_divisor = material_color.ambient.w * object_color.xyz;
    let ambient_color = (light.ambient_color.xyz * material_color.ambient.xyz) * ambient_divisor;

    //diffuse
    let light_dir = normalize(-light.direction);
    let diffuse_strength = max(dot(in.world_normal, light_dir),0.0);
    let diffuse_divisor = material_color.diffuse.w * object_color.xyz;
    let diffuse_color = light.diffuse_color.xyz * (diffuse_strength * material_color.diffuse.xyz) * diffuse_divisor;

    //specular
    let view_dir = normalize(uniform.u_view_position - in.world_position);
    let reflect_dir = reflect(-light_dir, in.world_normal);

    let specular_strength = pow(max(dot(view_dir,reflect_dir), 0.0), material_color.factor.x);
    let specular_color = light.specular_color.xyz * (specular_strength * material_color.specular.xyz) * material_color.specular.w;

    let result = (ambient_color * light.ambient_color.w + diffuse_color * light.diffuse_color.w  + specular_color * light.specular_color.w);
    return vec4<f32>(result, material_color.factor.y);
}
