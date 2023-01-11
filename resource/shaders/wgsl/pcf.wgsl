// macro are not supported yet. Don't hardcode the desired shadow texture size.
//todo remove macro
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

   let base_uv = (vec2<f32>(base_u, base_v) - vec2<f32>(0.5)) * RCP_DEFAULT_SHADOW_MAP_SIZE;

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

fn sample_pcf_gaussian_9_tap(light_index: i32, shadow_coord: vec3<f32>, shadow_ddx : vec3<f32>, shadow_ddy : vec3<f32>){
       let texel_size = RCP_DEFAULT_SHADOW_MAP_SIZE;

       let reciever_plane = retriever_depth_plane_bias(shadow_ddx, shadow_ddy, texel_size);

       let uv = shadow_coord.xy * DEFAULT_SHADOW_MAP_SIZE;

       let base_u = floor(uv.x + 0.5);
       let base_v = floor(uv.y + 0.5);

       let s = (uv.x + 0.5 - base_u);
       let t = (uv.y + 0.5 - base_v);

       let base_uv = (vec2<f32>(base_u, base_v) - vec2<f32>(0.5)) * RCP_DEFAULT_SHADOW_MAP_SIZE;

       let uw = vec3<f32>(4.0 - 3.0 * s, 7.0, 1.0 + 3.0 * s);
       let vw = vec3<f32>(4.0 - 3.0 * t, 7.0, 1.0 + 3.0 * t);

       let u = vec3<f32>((3.0 - 2.0 * s) / uw.x - 2.0, (3.0 + s) / uw.y, s / uw.z + 2.0);
       let v = vec3<f32>((3.0 - 2.0 * t) / vw.x - 2.0, (3.0 + t) / vw.y, t / vw.z + 2.0);

       var visibility = 0.0;

       visibility += uw.x * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.x, v.x), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.y, v.x), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.z, v.x), light_index, shadow_coord.z, reciever_plane);

       visibility += uw.x * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.x, v.y), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.y, v.y), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.z, v.y), light_index, shadow_coord.z, reciever_plane);

       visibility += uw.x * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.x, v.z), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.y, v.z), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.z, v.z), light_index, shadow_coord.z, reciever_plane);

       return visibility / 144.0;
}


fn sample_pcf_gaussian_16_tap(light_index: i32, shadow_coord: vec3<f32>, shadow_ddx : vec3<f32>, shadow_ddy : vec3<f32>){
       let texel_size = RCP_DEFAULT_SHADOW_MAP_SIZE;

       let reciever_plane = retriever_depth_plane_bias(shadow_ddx, shadow_ddy, texel_size);

       let uv = shadow_coord.xy * DEFAULT_SHADOW_MAP_SIZE;

       let base_u = floor(uv.x + 0.5);
       let base_v = floor(uv.y + 0.5);

       let s = (uv.x + 0.5 - base_u);
       let t = (uv.y + 0.5 - base_v);

       let base_uv = (vec2<f32>(base_u, base_v) - vec2<f32>(0.5)) * RCP_DEFAULT_SHADOW_MAP_SIZE;

       let uw = vec4<f32>(5.0 * s - 6.0, 11.0 * s - 28.0, -(11.0 * s * 17.0), -(5.0 * s + 1.0));
       let uv = vec4<f32>(5.0 * t - 6.0, 11.0 * t - 28.0, -(11.0 * t * 17.0), -(5.0 * t + 1.0));

       let u = vec4<f32>((4.0 * s - 5.0) / uw.x - 3.0, (4.0 * s - 16.0) / uw.y - 1.0, -(7.0 * s + 5.0) / uw.z + 1.0, -s / uw.w + 3.0);
       let v = vec4<f32>((4.0 * t - 5.0) / vw.x - 3.0, (4.0 * t - 16.0) / uv.y - 1.0, -(7.0 * t + 5.0) / uv.z + 1.0, -s / uv.w + 3.0);

       var visibility = 0.0;

       visibility += uw.x * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.x, v.x), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.y, v.x), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.z, v.x), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.w * vw.x * sample_shadow_map(base_uv, vec2<f32>(u.w, v.x), light_index, shadow_coord.z, reciever_plane);

       visibility += uw.x * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.x, v.y), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.y, v.y), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.z, v.y), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.w * vw.y * sample_shadow_map(base_uv, vec2<f32>(u.w, v.y), light_index, shadow_coord.z, reciever_plane);

       visibility += uw.x * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.x, v.z), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.y, v.z), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.z, v.z), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.w * vw.z * sample_shadow_map(base_uv, vec2<f32>(u.w, v.z), light_index, shadow_coord.z, reciever_plane);

       visibility += uw.x * vw.w * sample_shadow_map(base_uv, vec2<f32>(u.x, v.w), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.y * vw.w * sample_shadow_map(base_uv, vec2<f32>(u.y, v.w), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.z * vw.w * sample_shadow_map(base_uv, vec2<f32>(u.z, v.w), light_index, shadow_coord.z, reciever_plane);
       visibility += uw.w * vw.w * sample_shadow_map(base_uv, vec2<f32>(u.w, v.w), light_index, shadow_coord.z, reciever_plane);


       return visibility / 2704.0;
}


fn sample_pcf_poisson_4_tap(light_index: i32, shadow_coord: vec3<f32>, shadow_ddx : vec3<f32>, shadow_ddy : vec3<f32>) -> f32{
    let texel_size = RCP_DEFAULT_SHADOW_MAP_SIZE;

    let reciever_plane = retriever_depth_plane_bias(shadow_ddx, shadow_ddy, texel_size);

    let shadow_depth = shadow_coord.z + reciever_plane.z;

    let rotation_matrix = rotate_poisson_disk(shadow_coord.xy);

    var visibility = 0.0;
    for (var i = 0u; i < 4u; i += 1u){
        //todo: khal 700.0 magic number should be customizable this represent how much the samples are spread out
        let offset = (poissonDisk[i] * rotation_matrix) / 700.0;

        let sample_depth = shadow_depth + dot(offset, reciever_plane);

        visibility +=  textureSampleCompareLevel(t_shadow, sampler_shadow, shadow_coord.xy + offset, light_index, sample_depth);
    }

    return visibility / 4.0;
}

fn sample_pcf_poisson_9_tap() -> f32{
    //todo: implement this khal
}

fn sample_pcf_poisson_16_tap() -> f32{
    //todo: implement this khal
}