// This will be used for shadow.

struct View{
    view_projection : mat3x4<f32>,
}

@group(0) @binding(0)
var<uniform> view : View;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
};


@vertex
fn vs_main(in : VertexInput) -> VertexOutput{
    var out : VertexOutput;
    out.position = view.view_projection * vec3(in.position, 1.0);
    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(in : VertexInput ) -> FragmentOutput{
    return FragmentOutput(vec3(0.0));
}

