struct MetaBalls {
    dimensions: vec2<f32>,
    // array stride has to be 16byte aligned
    positions: array<vec4<f32>, 4>,
    velocity: array<vec4<f32>, 4>,
    colors: array<vec4<f32>, 4>,
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
        let ball = balls.positions[i].xyz;
        let dist = formula(coord, ball);
        sum += dist;
    }
    if (sum <= 1.) {
        discard;
    }
    return sum;
}

fn circle(coord: vec2<f32>) -> vec4<f32> {
    for (var i = 0; i < 4; i++) {
        if (distance(balls.positions[i].xy, coord) <= 0.3) {
            return vec4<f32>(1.);
        }
    }
    return vec4<f32>(0.);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = fields(in.coord);
    return vec4<f32>(vec2<f32>(color) * in.coord, color, 1.0);
    // return circle(in.coord);
}

