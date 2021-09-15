//Constants
let FRAC_1_PI : f32 = 0.31830987;

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

[[block]]
struct Material{

    // Diffuse albedo for non-metallic surfaces, and specular color for metallic surfaces
    albedo : vec4<f32>,

    // Whether a surface appears to be dielectric (0.0) or conductor (1.0). Often used as a binary value (0 or 1)
    metallic : f32,

    // Perceived smoothness (0.0) or roughness (1.0) of a surface. Smooth surfaces exhibit sharp reflections
    roughness : f32,

    // index of refraction
    refraction : f32,

    // Defines how much of the ambient light is accessible to a surface point. It is a per-pixel shadowing factor between 0.0 and 1.0.
    ambient_occlusion : f32,

    // future implementation.
    // emissive : f32,
};


[[group(2), location(0)]]
var<uniform> material : Material;


// Normal distribution function (specular D)
fn d_ggx(n_dot_h : f32, roughness : f32) -> f32{

    let sqr_roughness : f32 = roughness * roughness;
    let f : f32 = (n_dot_h * sqr_roughness - n_dot_h) * n_dot_h + 1.0;

    return (sqr_roughness / (f * f)) * FRAC_1_PI;
}

fn v_smith_ggx_correlation(n_dot_l : f32, n_dot_v : f32, roughness : f32) -> f32{

    // Optimized version.
    let sqr_roughness : f32 = roughness * roughness;

    let lambda_ggxv : f32 = n_dot_l * sqrt((-n_dot_v * sqr_roughness + n_dot_v) * n_dot_v + sqr_roughness);
    let lambda_ggxl : f32 = n_dot_v * sqrt((-n_dot_l * sqr_roughness + n_dot_l) * n_dot_l + sqr_roughness);

    return 0.5 / (lambda_ggxv + lambda_ggxl);
}

fn f_schlick(u : f32, f0 : vec3<f32>, f90 : f32) -> vec3<f32>{
    return f0 + (f90 - f0) * pow(1.0 - u, 5.0);
}

fn f_schlick(u : f32, f0 : vec3<f32>) -> vec3<f32>{
    let f : f32 = pow(1.0 - u, 5.0);
    return f + f0 * (1.0 - f);
}

fn specular_brdf(f0 : vec3<f32>, roughness : f32, h : vec3<f32>, n_dot_v : f32,
            n_dot_l : f32, n_dot_h : f32, l_dot_h : f32, intensity : f32) -> vec3<f32>{

            // Normal distribution function
            let D : f32 = d_ggx(n_dot_h, roughness);
            let F : vec3<f32> = f_schlick(l_dot_h, f0);
            let V : f32 = v_smith_ggx_correlation(n_dot_v, n_dot_l, roughness);

            return (intensity * D * V) * F;
}

fn fd_burley(roughness :f32, n_dot_v : f32, n_dot_l : f32, l_dot_h : f32) -> f32{

    let energy_bias : f32 = mix(0.0, 0.5, roughness);
    let energy_factor : f32 = mix(1.0, 1.0 / 1.51, roughness);
    let fd90 : f32 = energy_bias + 2.0 * l_dot_h * l_dot_h * roughness;
    let f0 : vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    let light_scatter : f32 = f_schlick(f0, fd90, n_dot_l).r;
    let view_scatter : f32 = f_schlick(f0, fd90, n_dot_v).r;
    return light_scatter + view_scatter * energy_factor;
}



fn diffuse_brdf(roughness : f32, n_dot_v : f32, n_dot_l : f32, l_dot_h : f32) -> f32{

    //add more stuff.

    return fd_burley(roughness, n_dot_v, n_dot_l, l_dot_h);

}


fn calculate_perceptual_roughness(roughness : f32) -> f32{
    let clamped_roughness : f32 = clamp(roughness, 0.045, 1.0);
    return clamped_roughness * clamped_roughness;
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

    let output_color = material.albedo;

    //basic color texture map.

    let diffuse_color : vec3<f32> = (1.0 - material.metallic) * output_color.rgb;

    // f0(n_ior) = (n_ior - 1.0)^2 / (n_ior + 1.0)^2
    // We can get the index of refraction from obj wave front file from Ni
    // Ni is the optical density for its surface also know as index of refraction.
    // https://en.wikipedia.org/wiki/Wavefront_.obj_file

    // We also enable support for ior for gltf file through features KHR_materials_ior
    // https://github.com/gltf-rs/gltf/pull/293

    // Collada support ior through either PhongEffect or LamberEffect
    // https://docs.rs/collada/0.14.0/collada/index.html?search=index

    let f0 : f32 = (pow((material.refraction - 1.0), 2.0) / pow((material.refraction + 1.0), 2.0)) *
                    (1.0 - material.metallic) + output_color.rgb * material.metallic;


    let perceptual_roughness : f32 = calculate_perceptual_roughness(material.roughness);

    //template
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}