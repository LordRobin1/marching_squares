#![allow(unused)]
#![allow(dead_code)]

// use colored::*;
use softbuffer::GraphicsContext;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {
                let size = window.inner_size();
                let mid = Point {
                    x: size.width as i32 / 2,
                    y: size.height as i32 / 2,
                };
                let buffer = ring_shader(&size, mid, &100.0, &10.0);

                graphics_context.set_buffer(&buffer, size.width as u16, size.height as u16);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn ring_shader(size: &PhysicalSize<u32>, center: Point, radius: &f32, thickness: &f32) -> Vec<u32> {
    let (width, height) = (size.width, size.height);

    (0..((width * height) as usize))
        .map(|index| {
            let y = (index / (width as usize)) as i32;
            let x = (index % (width as usize)) as i32;
            let point = Point { x, y };
            let distance = center.distance(&point);

            let in_circle = step(distance, *radius);
            let in_ring = 1. - step(distance, (*radius - thickness));

            let rgb = (255.0 * in_circle * in_ring) as u32;

            let mut color = rgb;
            color = (color << 8) + rgb;
            color = (color << 8) + rgb;
            color
        })
        .collect::<Vec<_>>()
}

fn circle_shader(size: &PhysicalSize<u32>, center: Point, radius: &f32) -> Vec<u32> {
    let (width, height) = (size.width, size.height);

    (0..((width * height) as usize))
        .map(|index| {
            let y = (index / (width as usize)) as i32;
            let x = (index % (width as usize)) as i32;
            let point = Point { x, y };
            let distance = center.distance(&point);

            let in_circle = step(distance, *radius);

            let rgb = (255.0 * in_circle) as u32;
            let mut color = rgb;
            color = (color << 8) + rgb;
            color = (color << 8) + rgb;
            color
        })
        .collect::<Vec<_>>()
}

fn step(value: f32, edge: f32) -> f32 {
    match value < edge {
        true => 1.,
        _ => 0.,
    }
}

fn distance_from_center(width: &u32, height: &u32) -> Vec<u32> {
    let origin = Point { x: 0, y: 0 };
    let mid = Point {
        x: *width as i32 / 2,
        y: *height as i32 / 2,
    };
    let max_dist = origin.distance(&mid);

    (0..((width * height) as usize))
        .map(|index| {
            let y = (index / (*width as usize)) as i32;
            let x = (index % (*width as usize)) as i32;
            let point = Point { x, y };
            let distance = mid.distance(&point);
            let mapped_dist = distance / max_dist;

            let rgb = (255.0 * mapped_dist) as u32;
            let mut color = rgb;
            color = (color << 8) + rgb;
            color = (color << 8) + rgb;
            color
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, point: &Point) -> f32 {
        let d_x = (self.x - point.x) as f32;
        let d_y = (self.y - point.y) as f32;
        (d_x * d_x + d_y * d_y).sqrt()
    }
}
