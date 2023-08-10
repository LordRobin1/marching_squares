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
    pub fn distance(&self, point: &Point) -> f32 {
        let d_x = self.x as f32 - point.x as f32;
        let d_y = self.y as f32 - point.y as f32;
        (d_x * d_x + d_y * d_y).sqrt()
    }
    pub fn in_range(&self, point: &Point, range: f32) -> bool {
        let d_x = self.x as f32 - point.x as f32;
        let d_y = self.y as f32 - point.y as f32;
        (d_x * d_x + d_y * d_y) < range.powf(2.)
    }
    pub fn clamp(&mut self, min: f32, max: f32) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
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
    /// Will return 0 if alpha == 0
    pub fn as_u32(&self) -> u32 {
        if self.a == 0. {
            0
        } else {
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
                self.r += color.r;
                self.g += color.g;
                self.b += color.b;
                self.a += color.a;
            }
        }
    }
    pub fn mult(&mut self, factor: f32) -> Color {
        Color {
            r: self.r * factor,
            g: self.r * factor,
            b: self.b * factor,
            a: self.a * factor,
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
