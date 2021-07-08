//Vertex Shader
[[block]]
struct Uniforms{
    u_view_position : vec3<f32>;
    view_proj : mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniform : Uniforms;


[[block]]
struct Light{
    position : vec3<f32>;
    color : vec3<f32>;
};

[[group(1), binding(0)]]
var<uniform> light : Light;

struct VertexOutput{
    [[builtin(position)]] light_position : vec4<f32>;
    [[location(0)]] color : vec3<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index : u32) -> VertexOutput {
    let scale = 0.25;
    var out : VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

    let model = vec3<f32>(x,y, 0.0);

    out.light_position = uniform.view_proj * vec4<f32>(model * scale + light.position, 1.0);
    out.color = light.color;
    return out;
}


[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec4<f32>{
    return vec4<f32>(in.color, 1.0);
}