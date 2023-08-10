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

    pub fn update(&mut self, size: PhysicalSize<u32>, delta_time: f32) {
        let (width, height) = (size.width as f32, size.height as f32);
        self.position = self.position.add(&self.velocity.mult(delta_time));

        if self.position.x >= width || self.position.x <= 0. {
            flip(&mut self.velocity, Axis::Vertical);
        }
        if self.position.y >= height || self.position.y <= 0. {
            flip(&mut self.velocity, Axis::Horizontal);
        }
        // println!("{}. pos: {:?}", i, self.positions[i]);
        // println!("{}. self.pos: {:?}", i, self.positions[i]);
    }
}

fn flip(velocity: &mut Point, axis: Axis) {
    match axis {
        Axis::Vertical => velocity.x = -velocity.x,
        Axis::Horizontal => velocity.y = -velocity.y,
    }
}
