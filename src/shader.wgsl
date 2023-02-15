// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) rotation: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

fn rotate(uv: vec2<f32>, rotation: f32) -> vec2<f32> {
    let cosa: f32 = cos(rotation);
    let sina: f32 = sin(rotation);

    return vec2(
      cosa * uv.x - sina * uv.y,
      cosa * uv.y + sina * uv.x,
    );
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    let rotate_coords: vec2<f32> = vec2(model.position.x, model.position.y);
    out.clip_position = vec4<f32>(rotate(rotate_coords, model.rotation), model.position.z, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
