// macro are not supported yet.
#import "shadow_common.wgsl"
#define DEFAULT_SHADOW_MAP_SIZE = 1024
#define RCP_DEFAULT_SHADOW_MAP_SIZE = 0.0009765625;

// Calculate the Base UV, S, and T for pcf gaussin NxN
fn pcf_gaussian(shadow_coord : vec3<f32>, shadow_coord_ddx : vec3<f32>, shadow_coord_ddy : vec3<f32>) -> vec4<f32>{

    let texel_size = RCP_DEFAULT_SHADOW_MAP_SIZE;

    let reciever_plane_bias = retriever_depth_plane_bias(shadow_coord_ddx, shadow_coord_ddy, texel_size);

    let depth_val = shadow_coord.z + reciever_plane_bias.z;

    let uv = shadow_coord.xy * DEFAULT_SHADOW_MAP_SIZE;

   let base_u = floor(uv.x + 0.5);
   let base_v = floor(uv.y + 0.5);

   let s = (uv.x + 0.5 - base_u);
   let t = (uv.y + 0.5 - base_v);

   let base_uv = (vec2(base_u, base_v) - vec2(0.5)) * texel_size;

    return vec4<f32>(base_uv.x,base_uv.y,s,t );
}


fn sample_pcf_gaussian_4_tap(){
    //todo
}

fn sample_pcf_gaussin_9_tap(){
    //todo
}

// We don't need one for 16 tap that a bit over the top.