use pixel_lib::{ColorMode::*, *};
use winit::dpi::PhysicalSize;

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
            .clamp(0., width, 0., height);

        if self.position.x >= width || self.position.x <= 0. {
            flip(&mut self.velocity, Axis::Vertical);
        }
        if self.position.y >= height || self.position.y <= 0. {
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
