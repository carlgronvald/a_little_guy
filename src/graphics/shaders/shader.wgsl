// Vertex shader


[[block]]
struct Uniforms {
    x_scale : f32;
    y_scale : f32;
    camera_offset : vec2<f32>;
};
[[group(1), binding(0)]]
var<uniform> uniforms : Uniforms;


struct VertexInput {
    [[location(0)]] position : vec3<f32>;
    [[location(1)]] tex_coords : vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position : vec4<f32>;
    [[location(0)]] tex_coords : vec2<f32>;
};


[[stage(vertex)]]
fn vs_main(
   model : VertexInput
) -> VertexOutput {
    var out : VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>((model.position.x + uniforms.camera_offset.x) * uniforms.x_scale, (model.position.y + uniforms.camera_offset.y) * uniforms.y_scale, model.position.z,1.0);
    return out;
}

// Fragment shader

// TODO: I DON'T UNDERSTAND THIS!
[[group(0), binding(0)]]
var t_diffuse : texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse : sampler;

[[stage(fragment)]]
fn fs_main(in : VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}