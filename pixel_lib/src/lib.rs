#[derive(Debug, Default)]
pub struct Pixel {
    pub pos: Point,
    pub color: Color,
}

#[derive(Debug, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
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
}

pub enum ColorMode {
    Lerp(f32),
    Additive,
}

/// rgb as f32s (could use u8s but then there's many casts)
/// a is pseudo value for color blending
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
            let mut color: u32 = self.r.round().clamp(0., 255.) as u32;
            color = (color << 8) + self.g.round().clamp(0., 255.) as u32;
            color = (color << 8) + self.b.round().clamp(0., 255.) as u32;
            color
        }
    }
    /// Lerps colors
    pub fn lerp(&mut self, color: &Color, weight: f32) {
        let w = weight.clamp(0.0, 1.);
        match (self.a.round() as u32, color.a.round() as u32) {
            (0, 0) => (),
            (0, _) => {
                //println!("0, _: color.r == {}", color.r);
                self.r = w * color.r;
                self.g = w * color.g;
                self.b = w * color.b;
                self.a = color.a;
            }
            // not sure about this one
            // (_, 0) => {
            //     self.r = self.r * (1. - w);
            //     self.g = self.g * (1. - w);
            //     self.b = self.b * (1. - w);
            // }
            _ => {
                self.r = self.r + w * (color.r - self.r);
                self.g = self.g + w * (color.g - self.g);
                self.b = self.b + w * (color.b - self.b);
            }
        }
    }
    pub fn add(&mut self, color: &Color) {
        match (self.a.round() as u32, color.a.round() as u32) {
            (0, 0) => (),
            (0, _) => *self = *color,
            (_, 0) => (),
            _ => {
                self.r = self.r + color.r;
                self.g = self.g + color.g;
                self.b = self.b + color.b;
            }
        }
    }
    pub fn factorize(&mut self, factor: f32) {
        self.r = self.r * factor;
        self.g = self.g * factor;
        self.b = self.b * factor;
    }
}

pub fn as_rgb(color: u32) -> Color {
    Color {
        r: ((color >> 16) & 0xff) as f32,
        g: ((color >> 8) & 0xff) as f32,
        b: (color & 0xff) as f32,
        a: 255., // default value for alpha, note that softbuffer doesn't have alpha so it's not represented in final u32
    }
}
