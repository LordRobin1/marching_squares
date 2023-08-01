#![allow(unused)]
#![allow(dead_code)]

use pixel_lib::{ColorMode::*, *};
use softbuffer::GraphicsContext;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent, WindowEvent::CursorMoved};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut fps: u128 = 0;
    let mut start = Instant::now();
    let mut delta_time = Default::default();
    let mut last_len = 0;

    let mut cursor = Point { x: 0, y: 0 };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                start = Instant::now();

                let size = window.inner_size();
                render(&mut graphics_context, &size, &cursor);

                // FPS
                delta_time = start.elapsed();
                print!("\r");
                // last_len = fps.to_string().len();
                fps = 1_000_000 / delta_time.as_micros();
                print!(
                    "FPS: {}, Cursor: {}, {}{}",
                    fps,
                    cursor.x,
                    cursor.y,
                    " ".repeat(50),
                );
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                window_id,
                event:
                    CursorMoved {
                        device_id,
                        position,
                        ..
                    },
            } if window_id == window.id() => {
                (cursor.x, cursor.y) = (position.x as u32, position.y as u32);
            }
            _ => {}
        }
    });
}

fn render(context: &mut GraphicsContext, size: &PhysicalSize<u32>, cursor: &Point) {
    let mut buffer = vec![0; (size.width * size.height) as usize];
    shaders(&mut buffer, size, cursor);
    context.set_buffer(&buffer, size.width as u16, size.height as u16);
}

fn shaders(buffer: &mut [u32], size: &PhysicalSize<u32>, cursor: &Point) {
    let (width, height) = (size.width, size.height);
    let mut pxl: Pixel;
    let circ_1 = *cursor;
    // let circ_1 = Point { x: 0, y: 0 }; // debug position
    let mid = Point {
        x: size.width / 2,
        y: size.height / 2,
    };
    let circ_2 = Point {
        x: mid.x + 50,
        y: mid.y,
    };
    let circ_3 = Point {
        x: mid.x - 50,
        y: mid.y,
    };
    // let thickness = 10.0;
    let radius = 100.0;
    let red = Color {
        r: 1.,
        g: 0.,
        b: 0.,
        a: 0.5,
    };
    let green = Color {
        r: 0.,
        g: 1.,
        b: 0.,
        a: 0.75,
    };
    let blue = Color {
        r: 0.,
        g: 0.,
        b: 1.,
        a: 1.,
    };

    for y in 0..height {
        for x in 0..width {
            pxl = Pixel {
                pos: Point { x, y },
                color: Color {
                    ..Default::default()
                },
            };
            circle_shader(&mut pxl, &circ_3, &radius, blue, ColorMode::Overlay);
            circle_shader(&mut pxl, &circ_2, &radius, green, ColorMode::Overlay);
            circle_shader(&mut pxl, &circ_1, &radius, red, ColorMode::Overlay);
            // ring_shader(&mut pxl, &ring, &radius, &thickness, ColorMode::Additive);
            // ring_shader(&mut pxl, &ring1, &radius, &thickness, ColorMode::Additive);

            buffer[(y * width + x) as usize] = pxl.color.as_u32();
        }
    }
}

fn circle_shader(
    pxl: &mut Pixel,
    center: &Point,
    radius: &f32,
    mut color: Color,
    col_mode: ColorMode,
) {
    let distance = center.distance(&pxl.pos);

    let in_circle = smooth_step(distance, *radius, *radius - 3.);

    match col_mode {
        Overlay => {
            color.lerp(&pxl.color, 1. - color.a * in_circle);
            pxl.color = color;
        }
        Lerp(x) => {
            color.lerp(&pxl.color, 1. - x * in_circle);
            pxl.color = color;
        }
        Additive => {
            color.factorize(in_circle);
            pxl.color.add(&color);
        }
        _ => (),
    }
}

fn ring_shader(
    pxl: &mut Pixel,
    center: &Point,
    radius: &f32,
    thickness: &f32,
    mut color: Color,
    col_mode: ColorMode,
) {
    let distance = center.distance(&pxl.pos);

    let in_circle = smooth_step(distance, *radius, *radius - 3.);
    let in_ring = 1. - smooth_step(distance, (*radius - thickness), (*radius - thickness) - 3.);
    let weight = in_circle * in_ring;

    match col_mode {
        Overlay => {
            color.lerp(&pxl.color, 1. - color.a * in_circle);
            pxl.color = color;
        }
        Lerp(x) => pxl.color.lerp(&color, 1. - x * weight),
        Additive => pxl.color.add(&color),
        _ => (),
    }
}

/// returns a value between 0. and 1.
fn step(value: f32, edge: f32) -> f32 {
    match value < edge {
        true => 1.,
        _ => 0.,
    }
}

/// returns a smooth value between 0. and 1.
fn smooth_step(value: f32, edge_0: f32, edge_1: f32) -> f32 {
    let x = ((value - edge_0) / (edge_1 - edge_0)).clamp(0., 1.);
    x * x * (3. - 2. * x)
}

fn dist_to_center(pxl: &Pixel, width: &u32, height: &u32) -> f32 {
    let origin = Point { x: 0, y: 0 };
    let mid = Point {
        x: *width / 2,
        y: *height / 2,
    };
    let max_dist = origin.distance(&mid);

    let distance = mid.distance(&pxl.pos);
    distance / max_dist
}
