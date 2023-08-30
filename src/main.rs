//
//  Implement Tinyrenderer in Rust for fun :)
//  Original here: https://github.com/ssloy/tinyrenderer
//
//  *Note*: Not worrying about being super idiomatic right now.
//  Mostly doing many iterations as brush strokes toward the end goal.
//
//  Currently on this step:
//  https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham’s-Line-Drawing-Algorithm#timings-fifth-and-final-attempt
//

use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

//
//  Line drawing to frame buffer
//  https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
//
fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    buffer: &mut [u32],
    r: u32,
    g: u32,
    b: u32,
) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    let distance_x = x1 - x0;
    let distance_y = y1 - y0;
    let distance_error = (distance_y).abs() * 2;
    let mut error = 0;
    let mut y = y0;
    let mut x = x0;
    while x <= x1 {
        let color = b | (g << 8) | (r << 16);
        if steep {
            buffer[x as usize * 2048 + y as usize] = color;
        } else {
            buffer[y as usize * 2048 + x as usize] = color;
        }
        error += distance_error;
        if error > distance_x {
            let error_diff = if y1 > y0 { 1 } else { -1 };
            y += error_diff;
            error -= distance_x * 2;
        }
        x += 1;
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let mut surface: Option<Surface> = None;
    let mut window: Option<Window> = None;

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.as_ref().unwrap().id() => {
                let (width, height) = {
                    let size = window.as_ref().unwrap().inner_size();
                    (size.width, size.height)
                };
                surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.as_mut().unwrap().buffer_mut().unwrap();

                line(13, 20, 80, 40, &mut buffer, 255, 255, 255);
                line(20, 13, 40, 80, &mut buffer, 255, 0, 0);
                line(80, 40, 13, 20, &mut buffer, 255, 0, 0);

                buffer.present().unwrap();
            }
            Event::NewEvents(StartCause::Init) => {
                window = Some(
                    WindowBuilder::new()
                        .with_title("Tiny Renderer")
                        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
                        .build(&event_loop)
                        .unwrap(),
                );
                let context = unsafe { Context::new(window.as_ref().unwrap()) }.unwrap();
                surface =
                    Some(unsafe { Surface::new(&context, &window.as_ref().unwrap()) }.unwrap());
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.as_ref().unwrap().id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    });
}
