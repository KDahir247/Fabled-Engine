// macro are not supported yet. Don't hardcode the desired shadow texture size.
#import "shadow_common.wgsl"
#define DEFAULT_SHADOW_MAP_SIZE = 1024
#define RCP_DEFAULT_SHADOW_MAP_SIZE = 0.0009765625; // 1.0 / 1024.0

// where index can be either the cascade shadow map index or the light array index
fn sample_shadow_map(base_uv : vec2<f32>, delta_uv : vec2<f32>,index : i32, depth : f32, receiver_depth_plane_bias : vec3<f32>) -> f32{
    let uv = base_uv + delta_uv * RCP_DEFAULT_SHADOW_MAP_SIZE;
    let z = depth + receiver_depth_plane_bias + dot(delta_uv * RCP_DEFAULT_SHADOW_MAP_SIZE, receiver_depth_plane_bias.xy);

    return textureSampleCompareLevel(t_shadow, sampler_shadow, uv , index, z);
}


fn sample_pcf_gaussian_4_tap(light_index: i32, shadow_coord: vec3<f32>, shadow_ddx : vec3<f32>, shadow_ddy : vec3<f32>) -> f32{

   let texel_size = RCP_DEFAULT_SHADOW_MAP_SIZE;

   let reciever_plane = retriever_depth_plane_bias(shadow_ddx, shadow_ddy, texel_size);

   let uv = shadow_coord.xy * DEFAULT_SHADOW_MAP_SIZE;

   let base_u = floor(uv.x + 0.5);
   let base_v = floor(uv.y + 0.5);

   let s = (uv.x + 0.5 - base_u);
   let t = (uv.y + 0.5 - base_v);

   let base_uv = (vec2(base_u, base_v) - vec2(0.5)) * RCP_DEFAULT_SHADOW_MAP_SIZE;

   let uw = vec2<f32>(3.0 - 2.0 * s, 1.0 + 2.0 * s);
   let vw = vec2<f32>(3.0 - 2.0 * t, 1.0 + 2.0 * t);

   let u = vec2<f32>((2.0 - s) / uw.x - 1.0,  s / uw.y + 1.0);
   let v = vec2<f32>((2.0 - t) / vw.x - 1.0, t / vw.y + 1.0);

   var visibility = 0.0;

   visibility += uw.x * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.x, v.x), light_index, shadow_coord.z, reciever_plane);
   visibility += uw.y * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.y, v.x), light_index, shadow_coord.z, reciever_plane);
   visibility += uw.x * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.x, v.y), light_index, shadow_coord.z, reciever_plane);
   visibility += uw.y * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.y, v.y), light_index, shadow_coord.z, reciever_plane);

    return visibility / 16.0;
}

fn sample_pcf_gaussin_9_tap(){
    //todo
}

// We don't need one for 16 tap that a bit over the top.