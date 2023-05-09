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
        let size = window.inner_size();
        let mut buffer = vec![0; (size.width * size.height) as usize];

        render(&mut buffer, &size);

        match event {
            Event::MainEventsCleared => {
                graphics_context.set_buffer(&buffer, size.width as u16, size.height as u16);
            }
            Event::RedrawRequested(window_id) if window.id() == window_id => {
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

fn render(buffer: &mut [u32], size: &PhysicalSize<u32>) {
    let mid = Point {
        x: size.width as i32 / 2,
        y: size.height as i32 / 2,
    };
    let circle = Point {
        x: mid.x - 150,
        y: mid.y,
    };
    let ring = Point {
        x: mid.x + 150,
        y: mid.y,
    };
    let ring1 = Point {
        x: mid.x + 100,
        y: mid.y,
    };
    circle_shader(buffer, size, circle, &100.0);
    ring_shader(buffer, size, ring, &100.0, &10.0);
    ring_shader(buffer, size, ring1, &100.0, &10.0);
}

fn ring_shader(
    buffer: &mut [u32],
    size: &PhysicalSize<u32>,
    center: Point,
    radius: &f32,
    thickness: &f32,
) {
    let (width, height) = (size.width, size.height);

    for (index, elem) in buffer.iter_mut().enumerate() {
        let y = (index / (width as usize)) as i32;
        let x = (index % (width as usize)) as i32;
        let point = Point { x, y };
        let distance = center.distance(&point);

        let in_circle = smooth_step(distance, *radius, *radius - 3.);
        let in_ring = 1. - smooth_step(distance, (*radius - thickness), (*radius - thickness) - 3.);

        let rgb = (255.0 * in_circle * in_ring) as u32;

        let mut color = rgb;
        color = (color << 8) + rgb;
        color = (color << 8) + rgb;

        *elem = color_lerp(color, *elem, 0.5)
    }
}

fn circle_shader(buffer: &mut [u32], size: &PhysicalSize<u32>, center: Point, radius: &f32) {
    let (width, height) = (size.width, size.height);

    for (index, elem) in buffer.iter_mut().enumerate() {
        let y = (index / (width as usize)) as i32;
        let x = (index % (width as usize)) as i32;
        let point = Point { x, y };
        let distance = center.distance(&point);

        let in_circle = smooth_step(distance, *radius, *radius - 3.);

        let rgb = (255.0 * in_circle) as u32;
        let mut color = rgb;
        color = (color << 8) + rgb;
        color = (color << 8) + rgb;

        *elem = color_lerp(color, *elem, 0.5)
    }
}

fn step(value: f32, edge: f32) -> f32 {
    match value < edge {
        true => 1.,
        _ => 0.,
    }
}

fn smooth_step(value: f32, edge_0: f32, edge_1: f32) -> f32 {
    let x = ((value - edge_0) / (edge_1 - edge_0)).clamp(0., 1.);
    x * x * (3. - 2. * x)
}

fn color_lerp(color_0: u32, color_1: u32, weight: f32) -> u32 {
    match (color_0, color_1) {
        (0, 0) => 0,
        (0, _) => color_1,
        (_, 0) => color_0,
        _ => {
            let (red_0, green_0, blue_0) = to_rgb(color_0);
            let (red_1, green_1, blue_1) = to_rgb(color_1);

            let red_lp = (red_0 + weight * (red_1 - red_0)) as u32;
            let green_lp = (green_0 + weight * (green_1 - green_0)) as u32;
            let blue_lp = (blue_0 + weight * (blue_1 - blue_0)) as u32;

            (255 << 24) + (red_lp << 16) + (green_lp << 8) + blue_lp
        }
    }
}

fn to_rgb(color: u32) -> (f32, f32, f32) {
    let red = (color >> 16) & 0xff;
    let green = (color >> 8) & 0xff;
    let blue = color & 0xff;

    (red as f32, green as f32, blue as f32)
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
