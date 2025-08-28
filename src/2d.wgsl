struct VertexInput {
    @location(0) color: vec4<f32>,
    @location(1) position: vec2<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> model: mat4x4<f32>;
@group(0) @binding(1)
var<uniform> view_projection: mat4x4<f32>;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = view_projection * model * vec4<f32>(input.position, 0.0, 1.0);
    out.color = input.color;
    out.uv = input.uv;
    return out;
}

@group(0) @binding(2)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(3)
var s_diffuse: sampler;
@group(0) @binding(4)
var<uniform> color: vec4<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv) * in.color * color;
}
