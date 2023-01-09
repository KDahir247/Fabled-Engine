struct Globals {
    view_proj: mat4x4<f32>,
    num_lights: vec4<u32>,
};

@group(0)
@binding(0)
var<uniform> u_globals: Globals;

struct Entity {
    world: mat4x4<f32>,
    color: vec4<f32>,
};

@group(1)
@binding(0)
var<uniform> u_entity: Entity;



 var<private> poissonDisk : array<vec2<f32>, 64>= array<vec2<f32>,64>(
    vec2<f32>(-0.5119625, -0.4827938),
    vec2<f32>(-0.2171264, -0.4768726),
    vec2<f32>(-0.7552931, -0.2426507),
    vec2<f32>(-0.7136765, -0.4496614),
    vec2<f32>(-0.5938849, -0.6895654),
    vec2<f32>(-0.3148003, -0.7047654),
    vec2<f32>(-0.42215, -0.2024607),
    vec2<f32>(-0.9466816, -0.2014508),
    vec2<f32>(-0.8409063, -0.03465778),
    vec2<f32>(-0.6517572, -0.07476326),
    vec2<f32>(-0.1041822, -0.02521214),
    vec2<f32>(-0.3042712, -0.02195431),
    vec2<f32>(-0.5082307, 0.1079806),
    vec2<f32>(-0.08429877, -0.2316298),
    vec2<f32>(-0.9879128, 0.1113683f),
    vec2<f32>(-0.3859636, 0.3363545),
    vec2<f32>(-0.1925334, 0.1787288),
    vec2<f32>(0.003256182, 0.138135),
    vec2<f32>(-0.8706837, 0.3010679),
    vec2<f32>(-0.6982038, 0.1904326),
    vec2<f32>(0.1975043, 0.2221317),
    vec2<f32>(0.1507788, 0.4204168),
    vec2<f32>(0.3514056, 0.09865579),
    vec2<f32>(0.1558783, -0.08460935),
    vec2<f32>(-0.068497f, 0.4461993),
    vec2<f32>(0.3780522, 0.3478679),
    vec2<f32>(0.3956799, -0.1469177),
    vec2<f32>(0.5838975, 0.1054943),
    vec2<f32>(0.6155105, 0.3245716),
    vec2<f32>(0.3928624, -0.4417621),
    vec2<f32>(0.1749884, -0.4202175),
    vec2<f32>(0.6813727, -0.2424808),
    vec2<f32>(-0.6707711, 0.4912741),
    vec2<f32>(0.0005130528, -0.8058334),
    vec2<f32>(0.02703013, -0.6010728),
    vec2<f32>(-0.1658188, -0.9695674),
    vec2<f32>(0.4060591, -0.7100726),
    vec2<f32>(0.7713396, -0.4713659),
    vec2<f32>(0.573212, -0.51544),
    vec2<f32>(-0.3448896, -0.9046497),
    vec2<f32>(0.1268544, -0.9874692),
    vec2<f32>(0.7418533, -0.6667366),
    vec2<f32>(0.3492522, 0.5924662),
    vec2<f32>(0.5679897, 0.5343465),
    vec2<f32>(0.5663417, 0.7708698),
    vec2<f32>(0.7375497, 0.6691415),
    vec2<f32>(0.2271994, -0.6163502),
    vec2<f32>(0.2312844, 0.8725659),
    vec2<f32>(0.4216993, 0.9002838),
    vec2<f32>(0.4262091, -0.9013284),
    vec2<f32>(0.2001408, -0.808381),
    vec2<f32>(0.149394, 0.6650763),
    vec2<f32>(-0.09640376, 0.9843736),
    vec2<f32>(0.7682328, -0.07273844),
    vec2<f32>(0.04146584, 0.8313184),
    vec2<f32>(0.9705266, -0.1143304),
    vec2<f32>(0.9670017, 0.1293385),
    vec2<f32>(0.9015037, -0.3306949),
    vec2<f32>(-0.5085648, 0.7534177),
    vec2<f32>(0.9055501, 0.3758393),
    vec2<f32>(0.7599946, 0.1809109),
    vec2<f32>(-0.2483695, 0.7942952),
    vec2<f32>(-0.4241052, 0.5581087),
    vec2<f32>(-0.1020106, 0.6724468),
    );


@vertex
fn vs_bake(@location(0) position: vec4<i32>) -> @builtin(position) vec4<f32> {
    return u_globals.view_proj * u_entity.world * vec4<f32>(position);
}

struct VertexOutput {
    @builtin(position) proj_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec4<f32>
};

@vertex
fn vs_main(
    @location(0) position: vec4<i32>,
    @location(1) normal: vec4<i32>,
) -> VertexOutput {
    let w = u_entity.world;
    let world_pos = u_entity.world * vec4<f32>(position);
    var result: VertexOutput;
    result.world_normal = mat3x3<f32>(w.x.xyz, w.y.xyz, w.z.xyz) * vec3<f32>(normal.xyz);
    result.world_position = world_pos;
    result.proj_position = u_globals.view_proj * world_pos;

    return result;
}

// fragment shader

struct Light {
    proj: mat4x4<f32>,
    pos: vec4<f32>,
    color: vec4<f32>,
};

@group(0)
@binding(1)
var<storage, read> s_lights: array<Light>;
@group(0)
@binding(1)
var<uniform> u_lights: array<Light, 10>; // Used when storage types are not supported
@group(0)
@binding(2)
var t_shadow: texture_depth_2d_array;
@group(0)
@binding(3)
var sampler_shadow: sampler_comparison;

fn compute_receiver_plane_depth_bias(shadow_ddx : vec3<f32>, shadow_ddy : vec3<f32>, bias_multiplier : f32, texel_size : f32) -> vec3<f32>{


let bias_uv = vec2(
	shadow_ddy.y * shadow_ddx.z - shadow_ddx.y * shadow_ddy.z,
	shadow_ddx.x * shadow_ddy.z - shadow_ddy.x * shadow_ddx.z,
 );


    let biasxy = bias_uv * bias_multiplier / ((shadow_ddx.x * shadow_ddy.y) - (shadow_ddx.y * shadow_ddy.x));


    let fract_sample_err = dot(vec2(1.0) * texel_size,abs(biasxy) );
    let biasz = min(fract_sample_err, 0.01);

    return vec3<f32>(biasxy.x, biasxy.y, biasz);

}

fn fetch_shadow(light_id: u32, shadow_coord: vec3<f32>, shadow_ddx : vec3<f32>, shadow_ddy : vec3<f32>) -> f32 {

    var visibility = 0.0;

    let texel_size = 1.0 / 512.0;

    let error_fraction_sampling = compute_receiver_plane_depth_bias(shadow_ddx, shadow_ddy, 1.0, 512.0).z;
    let depth = shadow_coord.z - error_fraction_sampling;
	// 9 tap
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(-texel_size, -texel_size) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(0.0, -texel_size) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(texel_size, -texel_size) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(-texel_size, 0.0) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(0.0, 0.0) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(texel_size, .00) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(-texel_size, texel_size) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(0.0, texel_size) , i32(light_id), depth);
    visibility += textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + vec2(texel_size, texel_size) , i32(light_id), depth);



    return visibility / 9.0;
}

let c_ambient: vec3<f32> = vec3<f32>(0.05, 0.05, 0.05);
let c_max_lights: u32 = 10u;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(vertex.world_normal);
    // accumulate color
    var color: vec3<f32> = c_ambient;
    for(var i = 0u; i < min(u_globals.num_lights.x, c_max_lights); i += 1u) {
        let light = s_lights[i];
        // project into the light space
        var shadow_coord = light.proj * vertex.world_position;

        let flip_correction = vec2<f32>(0.5, -0.5);
        let proj_correction = 1.0 / shadow_coord.w;


        let shadow_ddx = dpdx(shadow_coord.xyz);
        let shadow_ddy = dpdy(shadow_coord.xyz);
        let res = vec3(shadow_coord.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5), shadow_coord.z * proj_correction);

        let shadow = fetch_shadow(i, res, shadow_ddx, shadow_ddy);
        // compute Lambertian diffuse term
        let light_dir = normalize(light.pos.xyz - vertex.world_position.xyz);
        let diffuse = max(0.0, dot(normal, light_dir));
        // add light contribution
        color += shadow * diffuse * light.color.xyz;
    }
    // multiply the light by material color
    return vec4<f32>(color, 1.0) * u_entity.color;
}

// The fragment entrypoint used when storage buffers are not available for the lights
@fragment
fn fs_main_without_storage(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(vertex.world_normal);
    var color: vec3<f32> = c_ambient;
    for(var i = 0u; i < min(u_globals.num_lights.x, c_max_lights); i += 1u) {
        // This line is the only difference from the entrypoint above. It uses the lights
        // uniform instead of the lights storage buffer
        let light = u_lights[i];
	  var shadow_coord = light.proj * vertex.world_position;

        let flip_correction = vec2<f32>(0.5, -0.5);
        let proj_correction = 1.0 / shadow_coord.w;


        let shadow_ddx = dpdx(shadow_coord.xyz);
        let shadow_ddy = dpdy(shadow_coord.xyz);
        let res = vec3(shadow_coord.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5), shadow_coord.z * proj_correction);

        let shadow = fetch_shadow(i, res, shadow_ddx, shadow_ddy);
        let light_dir = normalize(light.pos.xyz - vertex.world_position.xyz);
        let diffuse = max(0.0, dot(normal, light_dir));
        color += shadow * diffuse * light.color.xyz;
    }
    return vec4<f32>(color, 1.0) * u_entity.color;
}
