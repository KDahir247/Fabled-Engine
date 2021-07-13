//---------------------------- Vertex Shader ----------------------------

let grid : array<vec3<f32>, 6> = array<vec3<f32>, 6>(
                                         vec3<f32>(1.0, 1.0, 0.0),
                                         vec3<f32>(-1.0, -1.0, 0.0),
                                         vec3<f32>(-1.0, 1.0, 0.0),
                                         vec3<f32>(-1.0, -1.0, 0.0),
                                         vec3<f32>(1.0, 1.0, 0.0),
                                         vec3<f32>(1.0, -1.0, 0.0)
                                         );


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


struct VertexOutput{
        [[builtin(position)]] v_position : vec4<f32>;
        [[location(0)]] position : vec3<f32>;
        [[location(1)]] near_point : vec3<f32>;
        [[location(2)]] far_point : vec3<f32>;
};


fn unproject_point(x : f32, y : f32, z : f32) -> vec3<f32>{

    let unprojected_point : vec4<f32> = uniform.inv_view * uniform.inv_proj * vec4<f32>(x,y,z, 1.0);

    return unprojected_point.xyz / unprojected_point.w;
}



[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> VertexOutput{


    let p : vec3<f32> = grid[vertex_index];
    var out : VertexOutput;
    out.near_point = unproject_point(p.x, p.y, 0.0).xyz;
    out.far_point = unproject_point(p.x, p.y, 1.0).xyz;
    out.position = p;
    out.v_position = vec4<f32>(p, 1.0);
    return out;
}


struct FragmentOutput{
     [[builtin(frag_depth)]] depth: f32;
     //[[builtin(front_facing)]] y: bool;
     [[location(0)]] color: vec4<f32>;
};

//---------------------------- Fragment Shader ----------------------------

fn checkerboard(R : vec2<f32>, scale : f32) -> f32{
    return (floor(R.x / scale) + floor(R.y / scale)) % 2.0;
}


fn computeDepth(pos : vec3<f32>) -> f32{
    let clip_space_pos = uniform.inv_proj * uniform.inv_view * vec4<f32>(pos.xyz, 1.0);

    let clip_space_depth = clip_space_pos.z / clip_space_pos.w;

    //todo don't hard code far is specified in setup.rs
    let far : f32 = 100.0;
    let near : f32 = 0.1;

    let depth = (((far-near) * clip_space_depth) + near + far) / 2.0;

    return depth;
}


[[stage(fragment)]]
fn fs_main( in : VertexOutput) -> FragmentOutput{

    let t : f32 = -in.near_point.y / (in.far_point.y - in.near_point.y);
    let r : vec3<f32> = in.near_point + t * (in.far_point);
    let c = checkerboard(r.xz, 1.0) * 0.3 +
            checkerboard(r.xz, 10.0) * 0.2 +
            checkerboard(r.xz, 100.0) * 0.1 +
            0.1;

    //c = c * float(t > 0);

    var out : FragmentOutput;
    out.depth = 1.0; //todo fix computeDepth(r);
    out.color = vec4<f32>(vec3<f32>(c/2.0 + 0.3), 1.0) * 0.3;

    if (r.x  > -0.05 && r.x < 0.05){
        out.color.x = 0.5;
    }

    if(r.z  > -0.05 && r.z < 0.05){
        out.color.z = 0.5;
    }


    //todo temporary solution
    if (t < 0.0){
        discard;
    }

    return out;
}