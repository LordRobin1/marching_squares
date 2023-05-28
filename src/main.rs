#![allow(unused)]
#![allow(dead_code)]

// use colored::*;
use softbuffer::GraphicsContext;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent, WindowEvent::CursorMoved};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ColorMode::*;

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
    let circ_1 = cursor;
    let mid = Point {
        x: size.width / 2,
        y: size.height / 2,
    };
    let circ_2 = &Point {
        x: mid.x + 50,
        y: mid.y,
    };
    let circ_3 = &Point {
        x: mid.x - 50,
        y: mid.y,
    };
    // let thickness = 10.0;
    let radius = 100.0;
    let mut r = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    let mut g = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    let mut b = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    for y in 0..height {
        for x in 0..width {
            pxl = Pixel {
                pos: Point { x, y },
                color: Color {
                    ..Default::default()
                },
            };
            circle_shader(&mut pxl, circ_1, &radius, &mut r, ColorMode::Lerp);
            circle_shader(&mut pxl, circ_2, &radius, &mut g, ColorMode::Lerp);
            circle_shader(&mut pxl, circ_3, &radius, &mut b, ColorMode::Lerp);
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
    color: &mut Color,
    col_mode: ColorMode,
) {
    let distance = center.distance(&pxl.pos);

    let in_circle = smooth_step(distance, *radius, *radius - 3.);

    match col_mode {
        Lerp => pxl.color.lerp(color, in_circle),
        Additive => pxl.color.add(color),
        _ => (),
    }
}

fn ring_shader(
    pxl: &mut Pixel,
    center: &Point,
    radius: &f32,
    thickness: &f32,
    color: &mut Color,
    col_mode: ColorMode,
) {
    let distance = center.distance(&pxl.pos);

    let in_circle = smooth_step(distance, *radius, *radius - 3.);
    let in_ring = 1. - smooth_step(distance, (*radius - thickness), (*radius - thickness) - 3.);
    let weight = in_circle * in_ring;

    match col_mode {
        Lerp => pxl.color.lerp(color, weight),
        Additive => pxl.color.add(color),
        _ => (),
    }
}

fn step(value: f32, edge: f32) -> f32 {
    match value < edge {
        true => 1.,
        _ => 0.,
    }
}

/// returns a value between 0. and 1.
fn smooth_step(value: f32, edge_0: f32, edge_1: f32) -> f32 {
    let x = ((value - edge_0) / (edge_1 - edge_0)).clamp(0., 1.);
    x * x * (3. - 2. * x)
}

fn as_rgb(color: u32) -> Color {
    Color {
        r: ((color >> 16) & 0xff) as u8,
        g: ((color >> 8) & 0xff) as u8,
        b: (color & 0xff) as u8,
        a: 255, // default value for alpha, note that softbuffer doesn't have alpha so it's not represented in final u32
    }
}

// not adapted to new shader system yet
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

#[derive(Debug, Default)]
struct Pixel {
    pos: Point,
    color: Color,
}

#[derive(Debug, Default)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance(&self, point: &Point) -> f32 {
        let d_x = self.x as f32 - point.x as f32;
        let d_y = self.y as f32 - point.y as f32;
        (d_x * d_x + d_y * d_y).sqrt()
    }
    fn in_range(&self, point: &Point, range: f32) -> bool {
        let d_x = self.x as f32 - point.x as f32;
        let d_y = self.y as f32 - point.y as f32;
        (d_x * d_x + d_y * d_y) < range.powf(2.)
    }
}

enum ColorMode {
    Lerp,
    Additive,
}

/// rgb as u8s
/// a is pseudo value for color blending
/// color will finally be black if a == 0
#[derive(Debug, Default, Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Will return 0 if alpha == 0
    fn as_u32(&self) -> u32 {
        if self.a == 0 {
            0
        } else {
            let mut color: u32 = self.r as u32;
            color = (color << 8) + self.g as u32;
            color = (color << 8) + self.b as u32;
            color
        }
    }
    fn lerp(&mut self, color: &Color, weight: f32) {
        match (self.a, color.a) {
            (0, 0) => (),
            (0, _) => {
                //println!("0, _: color.r == {}", color.r);
                self.r = (color.r as f32 * weight) as u8;
                self.g = (color.g as f32 * weight) as u8;
                self.b = (color.b as f32 * weight) as u8;
                self.a = color.a;
            }
            (_, 0) => {
                self.r = (self.r as f32 * (1. - weight)) as u8;
                self.g = (self.g as f32 * (1. - weight)) as u8;
                self.b = (self.b as f32 * (1. - weight)) as u8;
            }
            _ => {
                self.r = (self.r as f32 + weight * color.r.saturating_sub(self.r) as f32) as u8;
                self.g = (self.g as f32 + weight * color.g.saturating_sub(self.g) as f32) as u8;
                self.b = (self.b as f32 + weight * color.b.saturating_sub(self.b) as f32) as u8;
                self.a = (self.a as f32 + weight * color.a.saturating_sub(self.a) as f32) as u8;
            }
        }
    }
    fn add(&mut self, color: &Color) {
        match (self.a, color.a) {
            (0, 0) => (),
            (0, _) => *self = *color,
            (_, 0) => (),
            _ => {
                self.r = self.r.saturating_add(color.r);
                self.g = self.g.saturating_add(color.g);
                self.b = self.b.saturating_add(color.b);
                self.a = self.a.saturating_add(color.a);
            }
        }
    }
}
