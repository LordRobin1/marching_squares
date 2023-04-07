use colored::*;
use softbuffer::GraphicsContext;
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
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                let buffer = (0..((width * height) as usize))
                    .map(|index| {
                        let y = index / (width as usize);
                        let x = index % (width as usize);
                        let flipped_y = (height as usize - y) as f32;

                        let red = (255.0 * (x as f32 / width as f32)) as u32;
                        let green = (255.0 * (flipped_y / height as f32)) as u32;
                        let blue: u32 = 0;

                        let mut color = red;
                        color = (color << 8) + green;
                        color = (color << 8) + blue;

                        // println!("{}, {}", x, y);
                        // let print = format!("{:#010x}", color);
                        // println!("{}", print.truecolor(red as u8, green as u8, blue as u8));

                        color
                    })
                    .collect::<Vec<_>>();

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
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
