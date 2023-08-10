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
fn influence(pxl: vec2<f32>, ball: vec3<f32>) -> f32 {
    let ratio = ratio();
    return pow(ball.z, 2.) /
        (pow(pxl.x * ratio - ball.x * ratio, 2.) + pow(pxl.y - ball.y, 2.));
}

fn fields(coord: vec2<f32>) -> vec4<f32> {
    var sum: f32;
    var dists = array<f32, 4>();
    var col = vec4<f32>(0.);
    var min_dist = 10.;
    var min_i = 0;

    for (var i = 0; i < 4; i++) {
        let ball = balls.positions[i].xyz;
        // let dist = distance(ball.xy, coord);

        let influence = influence(coord, ball);
        sum += influence;
        col += balls.colors[i] * influence;    
        dists[i] = influence;

        if (dists[i] < min_dist) {
            min_dist = dists[i];
            min_i = i;
        }
    }

    if (sum <= 1.) {
        discard;
    }
    // for rings
    // if (sum >= 1.05) {
    //     discard;
    // }

    // load color of closest ball, garantueed to be affected
    // col = balls.colors[min_i];

    // for (var i = 0; i < 4; i++) {
    //     if (dists[i] < pow(balls.positions[i].z, 2.) && i != min_i) {
    //         col = mix(col, balls.colors[i], 0.5);
    //     }
    // }

    return col;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = fields(in.coord);
    return color;
}
