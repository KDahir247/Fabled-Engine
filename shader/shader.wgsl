//Blueprint

struct VertexOutput{
    [[builtin(position)]] v_position : vec4<f32>;
};


[[stage(vertex)]]
fn main() -> VertexOutput{
    var out : VertexOutput;
    out.v_position = vec4<f32>(0.0 ,0.0 ,0.0 ,0.0);
    return out;
}



[[stage(fragment)]]
fn main() -> [[location(0)]] vec4<f32>{

    return vec4<f32>(0.0, 0.0, 0.0, 0.0);

}
