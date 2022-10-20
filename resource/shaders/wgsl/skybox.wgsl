//---------------------------- Vertex Shader ----------------------------
[[block]]
struct Uniforms{
    u_view_position : vec4<f32>;
    view : mat4x4<f32>;
    proj : mat4x4<f32>;
    inv_proj : mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniform : Uniforms;

struct VertexOutput{
        [[builtin(position)]] v_position : vec4<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> VertexOutput{

    let x = f32(i32(vertex_index) - 1);
    let y = f32(i32(vertex_index & 1u) * 2 - 1);

    let pos = vec4<f32>(
        x * 400.0,
        y * 400.0,
        0.0,
        1.0
    );

    var out : VertexOutput;
    out.v_position = uniform.proj * uniform.view * pos;
    return out;
}




//---------------------------- Fragment Shader ----------------------------

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32>{

    return vec4<f32>(1.0, 0.0, 0.0, 1.0);

}