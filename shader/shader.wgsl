[[block]]
struct Uniforms{
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
};


[[stage(vertex)]]
fn main(in : VertexInput) -> VertexOutput{

    var out : VertexOutput;
    out.tex_coord = in.tex_coord;
    out.v_position = uniform.view_proj * vec4<f32>(in.position,1.0);
    return out;
}

[[group(0), binding(0)]] var t_diffuse : texture_2d<f32>;
[[group(0), binding(1)]] var t_sampler : sampler;

[[stage(fragment)]]
fn main(in : VertexOutput) -> [[location(0)]] vec4<f32>{

    return textureSample(t_diffuse, t_sampler, in.tex_coord);

}
