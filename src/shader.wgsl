struct MetaBalls {
    dimensions: vec2<u32>,
    positions: array<vec4<f32>, 4>,
    strengths: vec4<f32>
}
@group(0) @binding(0)
var<uniform> balls: MetaBalls;

struct VertexInput {
    @builtin(vertex_index) index: u32,
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) coord: vec2<f32>,
    @location(1) color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.coord = model.position.xy;
    out.clip_position = vec4<f32>(out.coord, 0.0, 1.0);
    return out;
}

fn in_circle(pos: vec2<f32>) -> vec4<f32> {
    let center = vec2<f32>(0.,0.);
    let radius = .5;
    var color = vec4<f32>(1.,1.,1.,1.);
    let dist = distance(center, pos);
    if (dist > radius) {
        discard;
    }
    let smoothing = smoothstep(radius, radius - .004, dist);
    color.a *= smoothing;
    return color;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = in_circle(in.coord);
    return color;
}

