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
