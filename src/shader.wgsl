@group(0) @binding(0)
var input: texture_storage_2d<rgba8unorm, write>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<u32> {
    let coords = vec2<u32>(in.clip_position.xy);
    let color = textureLoad(input, coords, 0u);
    color.w = 1u;
    return color;
}