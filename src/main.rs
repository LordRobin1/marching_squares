#![allow(unused)]
#![allow(dead_code)]

use pixel_lib::{Color, ColorMode::*, *};
use softbuffer::GraphicsContext;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
use winit::event::{Event, WindowEvent, WindowEvent::CursorMoved};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod balls;
mod square;
use crate::balls::*;
use crate::square::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut fps: u128 = 0;
    let mut start = Instant::now();
    let mut delta_time = Default::default();
    let mut last_len = 0;

    let mut cursor = Point { x: 0., y: 0. };

    let mut state = State::new(&window.inner_size(), graphics_context);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                start = Instant::now();

                let size = window.inner_size();

                state.render();
                state.update(&cursor, delta_time);

                // FPS
                let time = start.elapsed();
                delta_time = time.as_micros() as f32 / 1_000_000.0;
                print!("\r");
                // last_len = fps.to_string().len();
                fps = 1_000_000 / time.as_micros();
                print!(
                    "FPS: {}, Cursor: {}, {}{}",
                    fps,
                    cursor.x,
                    cursor.y,
                    " ".repeat(50),
                );
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
            } if window_id == window.id() => {
                state.resize(size, delta_time);
            }
            Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    },
            } if window_id == window.id() => {
                if let VirtualKeyCode::Space = input.virtual_keycode.unwrap() {
                    state.update = !state.update;
                }
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(physical_size) => state.resize(physical_size, delta_time),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(*new_inner_size, delta_time)
                }
                WindowEvent::CursorMoved { position, .. } => {
                    (cursor.x, cursor.y) = (position.x as f32, position.y as f32)
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        },
                    ..
                } => state.update = !state.update,
                _ => {}
            },
            _ => {}
        }
    });
}

struct State {
    balls: Vec<Ball>,
    buffer: Vec<u32>,
    context: GraphicsContext,
    size: (f32, f32),
    grid_res: u32,
    cursor: Point,
    update: bool,
}

impl State {
    fn new(size: &PhysicalSize<u32>, context: GraphicsContext) -> Self {
        let mid = Point {
            x: size.width as f32 / 2.,
            y: size.height as f32 / 2.,
        };
        let p1 = Point {
            x: mid.x + 70.,
            y: mid.y,
        };
        let p2 = Point {
            x: mid.x - 70.,
            y: mid.y,
        };
        let radius = 200.0;
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
                position: Point { x: 0., y: 0. },
                radius,
                velocity: Point { x: 75., y: -60. },
                color: red,
            },
            Ball {
                position: p1,
                radius,
                velocity: Point { x: 100., y: 60. },
                color: green,
            },
            Ball {
                position: p2,
                radius,
                velocity: Point { x: 140., y: -80. },
                color: blue,
            },
        ];
        Self {
            balls,
            buffer: vec![0; (size.width * size.height) as usize],
            context,
            size: (size.width as f32, size.height as f32),
            grid_res: 100,
            cursor: Point { x: 0., y: 0. },
            update: true,
        }
    }

    fn render(&mut self) {
        let (width, height) = self.size;
        // dbg!("start marching");
        self.marching_squares();
        self.context
            .set_buffer(&self.buffer, self.size.0 as u16, self.size.1 as u16);
        self.buffer = vec![0; (self.size.0 * self.size.1) as usize]
    }

    fn shade(&mut self) {
        let (width, height) = self.size;
        let mut pxl: Pixel;

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
                self.metaball_shader(&mut pxl, Overlay);
                self.buffer[(y as f32 * width + x as f32) as usize] = pxl.color.as_u32();
            }
        }
    }

    fn update(&mut self, cursor: &Point, delta_time: f32) {
        if !self.update {
            return;
        }
        for ball in self.balls.as_mut_slice() {
            ball.update(self.size, delta_time);
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>, delta_time: f32) {
        self.size = (size.width as f32, size.height as f32);
        self.buffer = vec![0; (size.width * size.height) as usize];
        let cursor = self.cursor;
        self.update(&cursor, delta_time);
    }

    /// marching squares algorithm to draw metaball contour
    fn marching_squares(&mut self) {
        let dimension = self.size.0 as u32 / self.grid_res;
        let (mut x, mut y) = (0u32, 0u32);
        while (0..self.size.1 as u32).contains(&y) {
            while (0..self.size.0 as u32).contains(&x) {
                // dbg!(x, y);
                let mut square = Square {
                    origin: Point {
                        x: x as f32,
                        y: y as f32,
                    },
                    dimension: dimension as f32,
                };
                square.march(&mut self.buffer, &self.balls, self.size.0 as u32);
                x += dimension;
            }
            y += dimension;
            x = 0;
        }
    }

    /// pixel based metaballs (not optimized because i'm too dumb for that polynomial stuff)
    fn metaball_shader(&mut self, pxl: &mut Pixel, col_mode: ColorMode) {
        let mut sum = 0.;
        for ball in self.balls.as_mut_slice() {
            let influence =
                { (ball.radius.powi(2) / pxl.pos.sq_distance(&ball.position)).clamp(0., 2.) };
            sum += influence.clamp(0., 1.);
            pxl.color.add(&ball.color.mult(influence));
        }
        if sum < 1.
        /*|| sum > 1.03*/
        {
            pxl.color = Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            };
        }
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
