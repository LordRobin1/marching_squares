use pixel_lib::{Color, *};
use softbuffer::GraphicsContext;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod balls;
mod square;
use crate::balls::*;
use crate::square::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let print = true;

    let mut delta_time = Default::default();

    let mut cursor = Point::origin();

    let mut state = State::new(&window.inner_size(), graphics_context);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                let start = Instant::now();

                state.render();
                state.update(&cursor, delta_time);

                let time = start.elapsed();
                delta_time = time.as_micros() as f32 / 1_000_000.0;

                // Print FPS
                if print {
                    print!("\r");
                    let fps = 1_000_000 / time.as_micros();
                    print!(
                        "FPS: {}, Grid Resolution: {}{}",
                        fps,
                        state.grid_res,
                        " ".repeat(20),
                    );
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
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    } => state.update = !state.update,
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    } => state.grid_res += 1,
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    } => state.grid_res = (state.grid_res - 1).clamp(1, u32::MAX),
                    _ => {}
                },
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
        let mid = Point::new_u(size.width / 2, size.height / 2);
        let p1 = Point::new_u(mid.x as u32 + 70, mid.y as u32);
        let p2 = Point::new_u(mid.x as u32 - 70, mid.y as u32);

        let radius = 0.1 * (size.width.pow(2) as f32 + size.height.pow(2) as f32).sqrt();

        let red = Color::new(1., 0., 0., 1.);
        let green = Color::new(0., 1., 0., 1.);
        let blue = Color::new(0., 0., 1., 1.);

        let balls = vec![
            Ball {
                position: Point::origin(),
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
            cursor: Point::origin(),
            update: true,
        }
    }

    fn render(&mut self) {
        self.marching_squares();
        self.context
            .set_buffer(&self.buffer, self.size.0 as u16, self.size.1 as u16);
        self.buffer = vec![0; (self.size.0 * self.size.1) as usize]
    }

    fn update(&mut self, cursor: &Point, delta_time: f32) {
        if !self.update {
            return;
        }
        self.balls[0].position = *cursor;
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

        let implicit_fn = |p: Point| -> f32 {
            let mut sum = 0.;
            for ball in &self.balls {
                let influence = (ball.radius.powi(2) / p.sq_distance(&ball.position)).clamp(0., 2.);
                sum += influence.clamp(0., 1.);
            }
            sum
        };

        while (0..self.size.1 as u32).contains(&y) {
            while (0..self.size.0 as u32).contains(&x) {
                let mut square =
                    Square::new(Point::new_u(x, y), dimension as f32, vec![], &implicit_fn);
                square.march(&mut self.buffer, self.size.0 as u32, self.size.1 as u32);
                x += dimension;
            }
            y += dimension;
            x = 0;
        }
    }
}
