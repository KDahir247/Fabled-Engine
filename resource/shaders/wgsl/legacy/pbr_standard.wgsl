// This shader assumes that the material is a conductor (metal) or semi-conductor
// non-anisotropy

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

    out.world_position = (transform.model * vec4<f32>(in.position, 1.0)).xyz;
    out.tex_coord = in.tex_coord;

    let normal : vec4<f32> = transform.model * vec4<f32>(in.normal, 1.0);
    out.world_normal = normal.xyz;

    out.world_tangent = transform.model * in.tangent;

    out.view_dir = in.position - uniform.u_view_position.xyz;

    out.v_position = uniform.proj * uniform.view * transform.model * vec4<f32>(in.position, 1.0);
    return out;
}



[[block]]
struct DirectionalLight{
    direction : vec4<f32>;

    // The color will be already premultiplied by the DirectionalLight Illuminance with exposure
    color : vec4<f32>;
};

[[group(2), binding(0)]]
var<uniform> sun : DirectionalLight;


[[block]]
struct Material{

    // Diffuse albedo for non-metallic surfaces, and specular color for metallic surfaces
    albedo : vec4<f32>;

    // Whether a surface appears to be dielectric (0.0) or conductor (1.0). Often used as a binary value (0 or 1)
    metallic : f32;

    // Perceived smoothness (0.0) or roughness (1.0) of a surface. Smooth surfaces exhibit sharp reflections
    roughness : f32;

    // index of refraction
    refraction : f32;

    // Defines how much of the ambient light is accessible to a surface point. It is a per-pixel shadowing factor between 0.0 and 1.0.
    ambient_occlusion : f32;

    // future implementation.
    // emissive : f32;
};


[[group(3), binding(0)]]
var<uniform> material : Material;

fn pow_5(x : f32) -> f32{
    return x * x * x * x * x;
}

fn env_dfg_polynomial(specular_color : vec3<f32>, gloss : f32, n_dot_v : f32) -> vec3<f32>{

    let x : f32  = gloss;
    let y : f32  = n_dot_v;

    let b1 : f32 = -0.1688;
    let b2 : f32 = 1.895;
    let b3 : f32 = 0.9903;
    let b4 : f32 = -4.853;
    let b5 : f32 = 8.404;
    let b6 : f32 = -5.069;

    var bias : f32;
    let un_saturated_bias : f32 = min( b1 * x + b2 * x * x, b3 + b4 * y + b5 * y * y + b6 * y * y * y );
    bias = max(0.0, min(1.0, un_saturated_bias));

    let d0 : f32 = 0.6045;
    let d1 : f32 = 1.699;
    let d2 : f32 = -0.5228;
    let d3 : f32 = -3.603;
    let d4 : f32 = 1.404;
    let d5 : f32 = 0.1939;
    let d6 : f32 = 2.661;

    let un_saturated_delta : f32 =  d0 + d1 * x + d2 * y + d3 * x * x + d4 * x * y + d5 * y * y + d6 * x * x * x;
    let delta : f32  = max(0.0, min(1.0, un_saturated_delta));

    let scale : f32 = delta - bias;

    bias = bias * max(0.0, min(1.0, 50.0 * specular_color.y));

    return specular_color * scale + bias;
}


// Normal distribution function (specular D)

// Represents the reflection of the ClearCoat Layer on the base material
// Generally isotropic non-metallic materials, namely ClearCoat Layer
fn d_gtr_1(roughness : f32, n_dot_h : f32) -> f32{
    let a2 : f32 = roughness * roughness;
    let cos_2_th : f32 = n_dot_h * n_dot_h;
    let den : f32 = (1.0 + (a2 - 1.0) * cos_2_th);

    return (a2 - 1.0) / (log(a2) * den) * FRAC_1_PI;
}

// Represents the reflection of the base material (Base Material)
// Can be anisotropic or isotropic metal or non-metal
fn d_gtr_2(roughness : f32, n_dot_h : f32) -> f32 {
    let a2 : f32 = roughness * roughness;
    let cos_2_th : f32 = n_dot_h * n_dot_h;
    let den  : f32 = (1.0 + (a2 - 1.0) * cos_2_th);

    return a2 / (log(a2) * den) * FRAC_1_PI;
}


fn g_smith_ggx_correlation(n_dot_l : f32, n_dot_v : f32, roughness : f32) -> f32{

    // Optimized version.
    let sqr_roughness : f32 = roughness * roughness;

    let lambda_ggxv : f32 = n_dot_l * sqrt((-n_dot_v * sqr_roughness + n_dot_v) * n_dot_v + sqr_roughness);
    let lambda_ggxl : f32 = n_dot_v * sqrt((-n_dot_l * sqr_roughness + n_dot_l) * n_dot_l + sqr_roughness);

    return 0.5 / (lambda_ggxv + lambda_ggxl);
}


//fn f_schlick(u : f32, f0 : vec3<f32>) -> vec3<f32>{
//    let f : f32 = pow((1.0 - u), 5.0);
//    return f + f0 * (1.0 - f);
//}

fn f_schlick(v_dot_h : f32, f0 : vec3<f32>, f90 : f32) -> vec3<f32>{
    return f0 + (f90 - f0) * pow_5(1.0 - v_dot_h);
}

fn specular_brdf(f0 : vec3<f32>, roughness : f32, h : vec3<f32>, n_dot_v : f32,
            n_dot_l : f32, n_dot_h : f32, l_dot_h : f32, intensity : f32) -> vec3<f32>{

            // Normal distribution function
            let D : f32 = d_gtr_2(roughness, n_dot_h);
            //todo temp we need f90
            let F : vec3<f32> = f_schlick(l_dot_h, f0, 1.0);
            let G : f32 = g_smith_ggx_correlation(n_dot_v, n_dot_l, roughness);

            return (intensity * D * G) * F;
}


fn fd_burley(roughness :f32, n_dot_v : f32, n_dot_l : f32, l_dot_h : f32) -> f32{

    let energy_bias : f32 = mix(0.0, 0.5, roughness);
    let energy_factor : f32 = mix(1.0, 1.0 / 1.51, roughness);
    let fd90 : f32 = energy_bias + 2.0 * l_dot_h * l_dot_h * roughness;
    let f0 : vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    //let light_scatter : f32 = f_schlick(f0, fd90, n_dot_l).r;
    //let view_scatter : f32 = f_schlick(f0, fd90, n_dot_v).r;
    return 1.0 + 1.0 * energy_factor;
}


fn diffuse_brdf(roughness : f32, n_dot_v : f32, n_dot_l : f32, l_dot_h : f32, diffuse_color : vec3<f32>) -> vec3<f32>{

    //add more stuff.

    return diffuse_color * fd_burley(roughness, n_dot_v, n_dot_l, l_dot_h);

}


fn calculate_perceptual_roughness(roughness : f32) -> f32{
    let clamped_roughness : f32 = clamp(roughness, 0.045, 1.0);
    return clamped_roughness * clamped_roughness;
}

// sRgb  to rgb
fn accurate_to_linear(gamma_col : vec3<f32>) -> vec3<f32>{
    let linear_rgb_lo : vec3<f32> = gamma_col / 12.92;

    let linear_rgb_hi_x : f32 =  pow((gamma_col.x + 0.055) / 1.055, 2.4);
    let linear_rgb_hi_y : f32 =  pow((gamma_col.y + 0.055) / 1.055, 2.4);
    let linear_rgb_hi_z : f32 =  pow((gamma_col.z + 0.055) / 1.055, 2.4);

    var linear_rgb : vec3<f32>;

    if (length(gamma_col) <= 0.04045){
        linear_rgb = linear_rgb_lo;
    }else{
        linear_rgb = vec3<f32>(linear_rgb_hi_x,linear_rgb_hi_y,linear_rgb_hi_z );
    }

    return linear_rgb;
}

// rgb to sRgb
fn accurate_to_gamma(linear_col : vec3<f32>) -> vec3<f32>{
    let s_rgb_lo : vec3<f32> = linear_col * 12.92;
    let s_rgb_hi_x : f32 = (pow(abs(linear_col.x), 1.0 / 2.4) * 1.055) - 0.055;
    let s_rgb_hi_y : f32 = (pow(abs(linear_col.y), 1.0 / 2.4) * 1.055) - 0.055;
    let s_rgb_hi_z : f32 = (pow(abs(linear_col.z), 1.0 / 2.4) * 1.055) - 0.055;

    var s_rgb : vec3<f32>;

    if (length(linear_col) <= 0.0031308){
        s_rgb = s_rgb_lo;
    }else{
        s_rgb = vec3<f32>(s_rgb_hi_x, s_rgb_hi_y, s_rgb_hi_z);
    }

    return s_rgb;
}


[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec4<f32>{

    let output_color = material.albedo;

    //todo basic color texture map. Make sure it is in linear space.


    let diffuse_color : vec3<f32> = (1.0 - material.metallic) * output_color.rgb;

    // f0(n_ior) = ((n_ior_1 - n_ior_2)^2 / (n_ior_1 + n_ior_2))^2
    // f0(n_ior) = ((n_ior - 1.0)^2 / (n_ior + 1.0))^2
    // We can get the index of refraction from obj wave front file from Ni
    // Ni is the optical density for its surface also know as index of refraction.
    // https://en.wikipedia.org/wiki/Wavefront_.obj_file

    // We also enable support for ior for gltf file through features KHR_materials_ior
    // https://github.com/gltf-rs/gltf/pull/293

    // Collada support ior through either PhongEffect or LamberEffect
    // https://docs.rs/collada/0.14.0/collada/index.html?search=index

    let f0 : vec3<f32> = (pow((material.refraction - 1.0), 2.0) / pow((material.refraction + 1.0), 2.0)) *
                   (1.0 - material.metallic) + output_color.rgb * material.metallic;


    let perceptual_roughness : f32 = calculate_perceptual_roughness(material.roughness);


    let incident_light = sun.direction.xyz;

    // We will expect the projection is perspective for test.
    // Using orthographic projection will produce incorrect result.
    let V : vec3<f32> = normalize(uniform.u_view_position.xyz - in.world_position);

    let L : vec3<f32> = normalize(-incident_light);
    let N : vec3<f32> = normalize(in.world_normal);

    let n_dot_v : f32 = max(dot(N, V), 0.0001);

    //Basic directional light.
    let n_dot_l : f32 = clamp(dot(N,L), 0.0, 1.0);

    // remap which was used by Crytek in Ryse.
    let gloss : f32 = pow((1.0 - perceptual_roughness * 0.7), 6.0);

    let diffuse_color : vec3<f32> = env_dfg_polynomial(diffuse_color, gloss, n_dot_v);
    let specular_color : vec3<f32> = env_dfg_polynomial(f0, gloss, n_dot_v);

    //todo finish this. (specular + diffuse)
    //let luminance = BSDF(...) * n_dot_l;

    //todo tone mapping.

    //todo correction if needed or vis versa.

    //todo convert linear space to gamma space by using accurate_to_gamma to precisely convert linear space to gamma space.

    //template
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}