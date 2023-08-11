#![allow(unused)]
#![allow(dead_code)]

use pixel_lib::{Color, ColorMode::*, *};
use softbuffer::GraphicsContext;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent, WindowEvent::CursorMoved};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod balls;
use crate::balls::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut fps: u128 = 0;
    let mut start = Instant::now();
    let mut delta_time = Default::default();
    let mut last_len = 0;

    let mut cursor = Point { x: 0., y: 0. };

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
                (cursor.x, cursor.y) = (position.x as f32, position.y as f32);
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
    let (width, height) = (size.width as f32, size.height as f32);
    let mut pxl: Pixel;
    let cursor_pos = *cursor;
    let mid = Point {
        x: width / 2.,
        y: height / 2.,
    };
    let p1 = Point {
        x: mid.x + 50.,
        y: mid.y,
    };
    let p2 = Point {
        x: mid.x - 50.,
        y: mid.y,
    };
    let radius = 100.0;
    let red = Color {
        r: 1.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
    let green = Color {
        r: 0.,
        g: 1.,
        b: 0.,
        a: 1.,
    };
    let blue = Color {
        r: 0.,
        g: 0.,
        b: 1.,
        a: 1.,
    };
    let mut balls = vec![
        Ball {
            position: cursor_pos,
            radius,
            velocity: Point { x: 50., y: 50. },
            color: red,
        },
        Ball {
            position: p1,
            radius,
            velocity: Point { x: 80., y: 30. },
            color: green,
        },
        Ball {
            position: p2,
            radius,
            velocity: Point { x: 110., y: -50. },
            color: blue,
        },
    ];

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            pxl = Pixel {
                pos: Point {
                    x: x as f32,
                    y: y as f32,
                },
                color: Color {
                    ..Default::default()
                },
            };
            metal_ball_shader(&mut pxl, &mut balls, Overlay);
            buffer[(y as f32 * width + x as f32) as usize] = pxl.color.as_u32();
        }
    }
}

fn metal_ball_shader(pxl: &mut Pixel, balls: &mut Vec<Ball>, col_mode: ColorMode) {
    let mut sum: f32 = 0.;
    for mut ball in balls {
        let influence = {
            (ball.radius.powi(2)
                / ((pxl.pos.x - ball.position.x).powi(2) + (pxl.pos.y - ball.position.y).powi(2)))
            .clamp(0., 1.)
        };
        sum += influence;
        pxl.color.add(&ball.color.mult(influence));
    }
    if sum < 1. {
        pxl.color = Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
        };
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

fn dist_to_center(pxl: &Pixel, width: &i32, height: &i32) -> f32 {
    let origin = Point { x: 0., y: 0. };
    let mid = Point {
        x: *width as f32 / 2.,
        y: *height as f32 / 2.,
    };
    let max_dist = origin.distance(&mid);

    let distance = mid.distance(&pxl.pos);
    distance / max_dist
}
