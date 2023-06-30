#![allow(unused)]
#![allow(dead_code)]

// use error_iter::ErrorIter as _;
use log::{debug, error};
use metaballs::run;
use pixel_lib::{ColorMode::*, *};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent, WindowEvent::CursorMoved};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    pollster::block_on(run());
}
