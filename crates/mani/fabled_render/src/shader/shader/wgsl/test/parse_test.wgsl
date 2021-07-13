//---------------------------- Vertex Shader ----------------------------
[[block]]
struct Uniforms{
    u_view_position : vec4<f32>;
    view : mat4x4<f32>;
    proj : mat4x4<f32>;
    inv_proj : mat4x4<f32>;
    inv_view : mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniform : Uniforms;

let grid : array<vec3<f32>, 6> = array<vec3<f32>, 6>(
                                         vec3<f32>(1.0, 1.0, 0.0),
                                         vec3<f32>(-1.0, -1.0, 0.0),
                                         vec3<f32>(-1.0, 1.0, 0.0),
                                         vec3<f32>(-1.0, -1.0, 0.0),
                                         vec3<f32>(1.0, 1.0, 0.0),
                                         vec3<f32>(1.0, -1.0, 0.0)
                                         );

struct VertexOutput{
        [[builtin(position)]] v_position : vec4<f32>;
        [[location(1)]] near_point : vec3<f32>;
        [[location(2)]] far_point : vec3<f32>;
};


fn unproject_point(x : f32, y : f32, z : f32) -> vec3<f32>{

    let viewInv : mat4x4<f32> = uniform.inv_view;
    let projInv : mat4x4<f32> = uniform.inv_proj;
    let unprojected_point = viewInv * projInv * vec4<f32>(x,y,z, 1.0);

    return unprojected_point.xyz / unprojected_point.w;
}



[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> VertexOutput{


    let p : vec3<f32> = grid[vertex_index];
    var out : VertexOutput;
    out.near_point = unproject_point(p.x, p.y, 0.0).xyz;
    out.far_point = unproject_point(p.x, p.y, 1.0).xyz;
    out.v_position = vec4<f32>(p, 1.0);
    return out;
}




//---------------------------- Fragment Shader ----------------------------

[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec4<f32>{

    let t : f32 = -in.near_point.y / (in.far_point.y - in.near_point.y);
    let result : f32 = max(0.0, t);
    return vec4<f32>(1.0, 0.0, 0.0, 1.0 * result);

}