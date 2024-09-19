struct Globals {
    ortho: mat4x4<f32>,
    transform: mat4x4<f32>,
    color: vec4<f32>,
    // has_texture: bool
};

@group(0) @binding(0) var<uniform> global: Globals;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(2) uv_0: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    // @location(1) tex_coords: vec2<f32>
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.color = global.color;
    // output.tex_coords = vertex.uv_0;
    output.clip_position = (global.ortho * global.transform) * vec4<f32>(vertex.position, 1.0);
    return output;
}

// @group(1) @binding(0)
// var t_diffuse: texture_2d<f32>;
// @group(1) @binding(1)
// var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) ->  @location(0) vec4<f32>  {
    // return global.has_texture ? in.color * textureSample(t_diffuse, s_diffuse, in.tex_coords) : in.color;
        return in.color;
}
