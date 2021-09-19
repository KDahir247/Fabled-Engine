//This is auto generated code. Do not modify code! Modification can break interpreted solution.
//Modify shader code if you know what you're doing.

#version 420 core
struct VertexOutput {
    vec4 v_position;
    vec3 position;
    vec3 near_point;
    vec3 far_point;
};

struct FragmentOutput {
    float depth;
    vec4 color;
};

uniform Uniforms_block_0 {
    vec4 u_view_position;
    mat4x4 view;
    mat4x4 proj;
    mat4x4 inv_proj;
    mat4x4 inv_view;
} _group_0_binding_0;

smooth out vec3 _vs2fs_location0;
smooth out vec3 _vs2fs_location1;
smooth out vec3 _vs2fs_location2;

vec3 unproject_point(float x, float y, float z) {
    mat4x4 _expr6 = _group_0_binding_0.inv_view;
    mat4x4 _expr8 = _group_0_binding_0.inv_proj;
    vec4 unprojected_point = ((_expr6 * _expr8) * vec4(x, y, z, 1.0));
    return (unprojected_point.xyz / vec3(unprojected_point.w));
}

float checkerboard(vec2 R, float scale) {
    return ((floor((R.x / scale)) + floor((R.y / scale))) % 2.0);
}

float computeDepth(vec3 pos) {
    mat4x4 _expr6 = _group_0_binding_0.proj;
    mat4x4 _expr8 = _group_0_binding_0.view;
    vec4 clip_space_pos = ((_expr6 * _expr8) * vec4(pos.xyz, 1.0));
    float clip_space_depth = (clip_space_pos.z / clip_space_pos.w);
    return clip_space_depth;
}

float computeLinearDepth(vec3 pos1) {
    mat4x4 _expr8 = _group_0_binding_0.proj;
    mat4x4 _expr10 = _group_0_binding_0.view;
    vec4 clip_space_pos = ((_expr8 * _expr10) * vec4(pos1.xyz, 1.0));
    float clip_space_depth = (((clip_space_pos.z / clip_space_pos.w) * 2.0) - 1.0);
    float linear_depth = (((2.0 * 0.1) * 100.0) / ((100.0 + 0.1) - (clip_space_depth * (100.0 - 0.1))));
    return (linear_depth / 100.0);
}

void main() {
    uint vertex_index = uint(gl_VertexID);
    VertexOutput out1;
    vec3 p = vec3(vec3(1.0, 1.0, 0.0), vec3(-1.0, -1.0, 0.0), vec3(-1.0, 1.0, 0.0), vec3(-1.0, -1.0, 0.0), vec3(1.0, 1.0, 0.0), vec3(1.0, -1.0, 0.0))[0];
    vec3 _expr10 = unproject_point(p.x, p.y, 0.0);
    out1.near_point = _expr10.xyz;
    vec3 _expr16 = unproject_point(p.x, p.y, 1.0);
    out1.far_point = _expr16.xyz;
    out1.position = p;
    out1.v_position = vec4(p, 1.0);
    VertexOutput _expr22 = out1;
    gl_Position = _expr22.v_position;
    _vs2fs_location0 = _expr22.position;
    _vs2fs_location1 = _expr22.near_point;
    _vs2fs_location2 = _expr22.far_point;
    return;
}

#version 420 core
struct VertexOutput {
    vec4 v_position;
    vec3 position;
    vec3 near_point;
    vec3 far_point;
};

struct FragmentOutput {
    float depth;
    vec4 color;
};

uniform Uniforms_block_0 {
    vec4 u_view_position;
    mat4x4 view;
    mat4x4 proj;
    mat4x4 inv_proj;
    mat4x4 inv_view;
} _group_0_binding_0;

smooth in vec3 _vs2fs_location0;
smooth in vec3 _vs2fs_location1;
smooth in vec3 _vs2fs_location2;
layout(location = 0) out vec4 _fs2p_location0;

vec3 unproject_point(float x, float y, float z) {
    mat4x4 _expr6 = _group_0_binding_0.inv_view;
    mat4x4 _expr8 = _group_0_binding_0.inv_proj;
    vec4 unprojected_point = ((_expr6 * _expr8) * vec4(x, y, z, 1.0));
    return (unprojected_point.xyz / vec3(unprojected_point.w));
}

float checkerboard(vec2 R, float scale) {
    return ((floor((R.x / scale)) + floor((R.y / scale))) % 2.0);
}

float computeDepth(vec3 pos) {
    mat4x4 _expr6 = _group_0_binding_0.proj;
    mat4x4 _expr8 = _group_0_binding_0.view;
    vec4 clip_space_pos = ((_expr6 * _expr8) * vec4(pos.xyz, 1.0));
    float clip_space_depth = (clip_space_pos.z / clip_space_pos.w);
    return clip_space_depth;
}

float computeLinearDepth(vec3 pos1) {
    mat4x4 _expr8 = _group_0_binding_0.proj;
    mat4x4 _expr10 = _group_0_binding_0.view;
    vec4 clip_space_pos = ((_expr8 * _expr10) * vec4(pos1.xyz, 1.0));
    float clip_space_depth = (((clip_space_pos.z / clip_space_pos.w) * 2.0) - 1.0);
    float linear_depth = (((2.0 * 0.1) * 100.0) / ((100.0 + 0.1) - (clip_space_depth * (100.0 - 0.1))));
    return (linear_depth / 100.0);
}

void main() {
    VertexOutput in1 = VertexOutput(gl_FragCoord, _vs2fs_location0, _vs2fs_location1, _vs2fs_location2);
    FragmentOutput out2;
    float t = ((- in1.near_point.y) / (in1.far_point.y - in1.near_point.y));
    vec3 r = (in1.near_point + (t * (in1.far_point - in1.near_point)));
    float _expr22 = checkerboard(r.xz, 1.0);
    float _expr27 = checkerboard(r.xz, 10.0);
    float _expr33 = checkerboard(r.xz, 100.0);
    float c = ((((_expr22 * 0.4) + (_expr27 * 0.2)) + (_expr33 * 0.1)) + 0.1);
    float _expr39 = computeLinearDepth(r);
    float fading_factor = max(0.0, (0.5 - _expr39));
    out2.depth = 1.0;
    vec3 checker_color = (vec3((((c / 4.0) + 0.3) + ((c / 2.0) + 0.3))) * 0.05);
    out2.color = (vec4(checker_color, 1.0) * 0.5);
    vec4 _expr67 = out2.color;
    out2.color.x = ((length(r.x) < 0.05) ? (1.0 * 0.5) : _expr67.x);
    vec4 _expr79 = out2.color;
    out2.color.z = ((length(r.z) < 0.05) ? (1.0 * 0.5) : _expr79.z);
    vec4 _expr88 = out2.color;
    out2.color = (_expr88 * ((t < 0.0) ? 0.0 : 1.0));
    vec4 _expr98 = out2.color;
    out2.color.w = (_expr98.w * fading_factor);
    FragmentOutput _expr101 = out2;
    gl_FragDepth = _expr101.depth;
    _fs2p_location0 = _expr101.color;
    return;
}

