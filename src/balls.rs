use pixel_lib::*;

enum Axis {
    Horizontal,
    Vertical,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ball {
    pub position: Point,
    pub radius: f32,
    pub velocity: Point,
    pub color: Color,
}

impl Ball {
    pub fn new(position: Point, radius: f32, velocity: Point, color: Color) -> Self {
        Self {
            position,
            radius,
            velocity,
            color,
        }
    }

    pub fn update(&mut self, size: (f32, f32), delta_time: f32) {
        let (width, height) = size;
        self.position = self
            .position
            .add(&self.velocity.mult(delta_time))
            // clamp, so that balls can't disappear on resize
            .clamp(
                self.radius,
                (width - self.radius).clamp(0.0, f32::INFINITY),
                self.radius,
                (height - self.radius).clamp(0.0, f32::INFINITY),
            );

        if self.position.x + self.radius >= width || self.position.x - self.radius <= 0.0 {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y + self.radius >= height || self.position.y - self.radius <= 0.0 {
            self.velocity.y = -self.velocity.y;
        }
        self.radius = 0.1 * (width.powi(2) + height.powi(2)).sqrt();
    }
}

impl PartialEq for Ball {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.radius == other.radius
            && self.velocity == other.velocity
            && self.color == other.color
    }
}
