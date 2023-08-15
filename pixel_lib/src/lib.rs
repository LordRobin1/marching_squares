#[derive(Debug, Default)]
pub struct Pixel {
    pub pos: Point,
    pub color: Color,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn origin() -> Self {
        Self { x: 0., y: 0. }
    }

    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }

    pub fn distance(&self, point: &Point) -> f32 {
        let d_x = self.x - point.x;
        let d_y = self.y - point.y;
        (d_x * d_x + d_y * d_y).sqrt()
    }

    pub fn sq_distance(&self, other: &Point) -> f32 {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        d_x * d_x + d_y * d_y
    }

    pub fn in_range(&self, point: &Point, range: f32) -> bool {
        let d_x = self.x - point.x;
        let d_y = self.y - point.y;
        (d_x * d_x + d_y * d_y) < range.powf(2.)
    }

    pub fn clamp(&mut self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Point {
        Point {
            x: self.x.clamp(min_x, max_x),
            y: self.y.clamp(min_y, max_y),
        }
    }

    pub fn mult(&self, factor: f32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub enum ColorMode {
    /// will lerp by provided value
    Lerp(f32),
    /// will lerp by alpha
    Overlay,
    Additive,
}

/// rgb as f32s (expects values from 0 - 1)
/// color will finally be black if a == 0
#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    /// Will return 0 if alpha == 0
    pub fn as_u32(&self) -> u32 {
        if self.a == 0. {
            0
        } else {
            let range = 0.0..=1.0;
            assert!(
                range.contains(&self.r),
                "self.r `{}` not in range 0.0..=1.0",
                self.r
            );
            assert!(
                range.contains(&self.g),
                "self.g `{}` not in range 0.0..=1.0",
                self.g
            );
            assert!(
                range.contains(&self.b),
                "self.b `{}` not in range 0.0..=1.0",
                self.b
            );
            assert!(
                range.contains(&self.a),
                "self.a `{}` not in range 0.0..=1.0",
                self.a
            );
            let mut color: u32 = (self.r * 255. * self.a) as u32;
            color = (color << 8) + (self.g * 255. * self.a) as u32;
            color = (color << 8) + (self.b * 255. * self.a) as u32;
            color
        }
    }
    /// Lerps colors
    pub fn lerp(&mut self, color: &Color, weight: f32) {
        let w = weight.clamp(0.0, 1.);
        match (self.a == 0., color.a == 0.) {
            (true, true) => (),
            (true, _) => {
                //println!("0, _: color.r == {}", color.r);
                self.r = w * color.r;
                self.g = w * color.g;
                self.b = w * color.b;
                self.a = w * color.a;
            }
            // not sure about this one
            // (_, 0) => {
            //     self.r = self.r * (1. - w);
            //     self.g = self.g * (1. - w);
            //     self.b = self.b * (1. - w);
            // }
            _ => {
                self.r += w * (color.r - self.r);
                self.g += w * (color.g - self.g);
                self.b += w * (color.b - self.b);
                self.a += w * (color.a - self.a);
            }
        }
    }
    pub fn add(&mut self, color: &Color) {
        match (self.a == 0., color.a == 0.) {
            (true, true) => (),
            (true, _) => *self = *color,
            (_, true) => (),
            _ => {
                self.r = (self.r + color.r).clamp(0., 1.);
                self.g = (self.g + color.g).clamp(0., 1.);
                self.b = (self.b + color.b).clamp(0., 1.);
                self.a = (self.a + color.a).clamp(0., 1.);
            }
        }
    }
    pub fn mult(&mut self, factor: f32) -> Color {
        Color {
            r: (self.r * factor).clamp(0., 1.),
            g: (self.g * factor).clamp(0., 1.),
            b: (self.b * factor).clamp(0., 1.),
            a: (self.a * factor).clamp(0., 1.),
        }
    }
    pub fn factorize(&mut self, factor: f32) {
        self.a *= factor;
    }
    pub fn as_rgb(color: u32) -> Color {
        Color {
            r: ((color >> 16) & 0xff) as f32 / 255.,
            g: ((color >> 8) & 0xff) as f32 / 255.,
            b: (color & 0xff) as f32 / 255.,
            a: 1., // default value for alpha, note that softbuffer doesn't have alpha so it's not represented in final u32
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

/// returns a value between 0. and 1.
pub fn step(value: f32, edge: f32) -> f32 {
    match value < edge {
        true => 1.,
        _ => 0.,
    }
}

/// returns a smooth value between 0. and 1.
pub fn smooth_step(value: f32, edge_0: f32, edge_1: f32) -> f32 {
    let x = ((value - edge_0) / (edge_1 - edge_0)).clamp(0., 1.);
    x * x * (3. - 2. * x)
}
