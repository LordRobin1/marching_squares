use std::f32::INFINITY;

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
            // clamp, so that balls can't disappearing on resize
            .clamp(
                0.,
                (width - self.radius).clamp(0., INFINITY),
                0.,
                (height - self.radius).clamp(0., INFINITY),
            );

        if self.position.x + self.radius >= width || self.position.x - self.radius <= 0. {
            flip(&mut self.velocity, Axis::Vertical);
        }
        if self.position.y + self.radius >= height || self.position.y - self.radius <= 0. {
            flip(&mut self.velocity, Axis::Horizontal);
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

fn flip(velocity: &mut Point, axis: Axis) {
    match axis {
        Axis::Vertical => velocity.x = -velocity.x,
        Axis::Horizontal => velocity.y = -velocity.y,
    }
}
