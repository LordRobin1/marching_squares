struct MetaBalls {
    dimensions: vec2<u32>,
    positions: array<vec4<f32>, 4>,
    strengths: vec4<f32>
}
@group(0) @binding(0)
var<uniform> balls: MetaBalls;

@group(0) @binding(1)
var output: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(16, 16)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dim = balls.dimensions;
    let coords = vec2<i32>(global_id.xy);

    if (coords.x >= dim.x || coords.y >= dim.y) {
        return;
    }

    let color = vec3<f32>(0.5, 0.5, 0.5);

    textureStore(output, coords.xy, vec4<f32>(color, 1.0));
}