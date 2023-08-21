use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

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
                for index in 0..(width * height) {
                    let y = index / width;
                    let x = index % width;
                    let red = x % 255;
                    let green = y % 255;
                    let blue = (x * y) % 255;

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

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
