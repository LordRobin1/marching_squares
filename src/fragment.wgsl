struct MetaBalls {
    dimensions: vec2<f32>,
    positions: array<vec3<f32>, 4>,
}
@group(0) @binding(0)
var<uniform> balls: MetaBalls;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) coord: vec2<f32>,
    @location(1) color: vec2<f32>,
}

fn ratio() -> f32 {
    return balls.dimensions.x / balls.dimensions.y;
}

// https://jamie-wong.com/2014/08/19/metaballs-and-marching-squares/
fn formula(pxl: vec2<f32>, ball: vec3<f32>) -> f32 {
    return pow(ball.z, 2.) / (pow(ball.x - pxl.x, 2.) + pow(ball.y - pxl.y, 2.));
}

fn fields(coord: vec2<f32>) -> f32 {
    var sum: f32;
    for (var i = 0; i < 4; i++) {
        let ball = balls.positions[i];
        let dist = formula(coord, ball);
        sum += dist;
    }
    if (sum <= 1.) {
        discard;
    }
    return sum;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = fields(in.coord);
    // if (color < 1.) {
    //     discard;
    // }
    return vec4<f32>(vec3<f32>(color), 1.0);
}

