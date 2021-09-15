[[block]]
struct Uniforms{
    u_view_position : vec4<f32>;
    view : mat4x4<f32>;
    proj : mat4x4<f32>;
    inv_proj : mat4x4<f32>;
    inv_view : mat4x4<f32>;
};

[[block]]
struct Transform{
    model : mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniform : Uniforms;

[[group(1), binding(0)]]
var<uniform> transform : Transform;

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
    [[location(3)]] world_tangent : vec4<f32>;
    [[location(4)]] view_dir : vec3<f32>;
};


[[stage(vertex)]]
fn vs_main(in : VertexInput) -> VertexOutput{
    var out : VertexOutput;

    out.world_position = transform.model * vec4<f32>(in.position, 1.0);
    out.tex_coord = in.tex_coord;

    let normal : vec4<f32> = (transform.model * vec4<f32>(in.normal, 1.0));
    out.world_normal = normal.xyz;

    out.world_tangent = transform.model * in.tangent;

    out.view_dir = in.position - uniform.u_view_position.xyz;

    out.v_position = uniform.proj * uniform.view * transform.model * vec4<f32>(in.position, 1.0);
    return out;
}

// sRgb  to rgb
fn accurate_to_linear(gamma_col : vec3<f32>) -> vec3<f32>{
    let linear_rgb_lo : vec3<f32> = gamma_col / 12.92;
    let linear_rgb_hi : vec3<f32> = pow((gamma_col + 0.055) / 1.055, 2.4);

    var linear_rgb : vec3<f32>;

    if (gamma_col <= 0.04045){
        linear_rgb = linear_rgb_lo;
    }else{
        linear_rgb = linear_rgb_hi;
    }

    return linear_rgb;
}

// rgb to sRgb
fn accurate_to_gamma(linear_col : vec<f32>) -> vec3<f32>{
    let s_rgb_lo : vec3<f32> = linear_col * 12.92;
    let s_rgb_hi : vec3<f32> = (pow(abs(linear_col), 1.0 / 2.4) * 1.055) - 0.055;

    var s_rgb : vec3<f32>;

    if (linear_col <= 0.0031308){
        s_rgb = s_rgb_lo;
    }else{
        s_rgb = s_rgb_hi;
    }

    return s_rgb;
}


[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec3<f32>{

    //template
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}