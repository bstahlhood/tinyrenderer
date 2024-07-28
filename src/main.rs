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
use std::rc::Rc;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
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
    stride: usize,
    scale: i32,
    r: u32,
    g: u32,
    b: u32,
) {
    x0 = x0 * scale;
    y0 = y0 * scale;
    x1 = x1 * scale;
    y1 = y1 * scale;
    let color = b | (g << 8) | (r << 16);
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
        if steep {
            buffer[x as usize * stride + y as usize] = color;
        } else {
            buffer[y as usize * stride + x as usize] = color;
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
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Tiny Renderer")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0 as u32, 800.0 as u32))
            .build(&event_loop)
            .unwrap(),
    );

    let context = Context::new(window.clone()).unwrap();
    let mut surface = Surface::new(&context, window.clone()).unwrap();

    let african_head = tobj::load_obj("models/african_head.obj", &tobj::LoadOptions::default());
    let (models, _) = african_head.expect("Failed to load OBJ file");

    let model = models.get(0).unwrap();

    // let event_loop = EventLoop::new();

    // let mut surface: Option<Surface> = None;
    // let mut window: Option<Window> = None;

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        println!("Width {} Height {}", size.width, size.height);
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {
                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();

                        let scale = window.scale_factor() as u32;
                        let stride = (window.inner_size().width) as usize;

                        let logical_width = width.get() / scale;
                        let logical_height = height.get() / scale;

                        println!("Logical Width {} Height {}", logical_width, logical_height);

                        let mesh = &model.mesh;

                        for face in (0..mesh.indices.len()).step_by(3) {
                            let vertices = &mesh.indices[face..face + 3];

                            println!("{:?}", vertices);

                            // for vertex in vertices {
                            let (v0, v1, v2) = {
                                let v0 = &mesh.positions
                                    [(vertices[0] * 3) as usize..(vertices[0] * 3 + 3) as usize];
                                let v1 = &mesh.positions
                                    [(vertices[1] * 3) as usize..(vertices[1] * 3 + 3) as usize];
                                let v2 = &mesh.positions
                                    [(vertices[2] * 3) as usize..(vertices[2] * 3 + 3) as usize];
                                (v0, v1, v2)
                            };

                            println!("Drawing line {:?} {:?} {:?}", v0, v1, v2);

                            let half_width = (logical_width as f32) / 2.;
                            let half_height = (logical_height as f32) / 2.;

                            let x0 = (v0[0] + 1.) * half_width;
                            let y0 = (v0[1] * -half_height) + half_height;
                            let mut x1 = (v1[0] + 1.) * half_width;
                            let mut y1 = (v1[1] * -half_height) + half_height;
                            let mut x2 = (v2[0] + 1.) * half_width;
                            let mut y2 = (v2[1] * -half_height) + half_height;

                            if x1 > logical_width as f32 - 1. {
                                x1 = logical_width as f32 - 1.;
                            }

                            if y1 > logical_height as f32 - 1. {
                                y1 = logical_height as f32 - 1.;
                            }

                            if x2 > logical_width as f32 - 1. {
                                x2 = logical_width as f32 - 1.;
                            }

                            if y2 > logical_height as f32 - 1. {
                                y2 = logical_height as f32 - 1.;
                            }

                            println!("Drawing line {x0} {y0} {x1} {y1}");

                            line(
                                x0 as i32,
                                y0 as i32,
                                x1 as i32,
                                y1 as i32,
                                &mut buffer,
                                stride,
                                scale as i32,
                                255,
                                255,
                                255,
                            );
                            line(
                                x1 as i32,
                                y1 as i32,
                                x2 as i32,
                                y2 as i32,
                                &mut buffer,
                                stride,
                                scale as i32,
                                255,
                                255,
                                255,
                            );
                            line(
                                x2 as i32,
                                y2 as i32,
                                x0 as i32,
                                y0 as i32,
                                &mut buffer,
                                stride,
                                scale as i32,
                                255,
                                255,
                                255,
                            );
                        }

                        // line(13, 20, 80, 40, &mut buffer, stride, 255, 255, 255);
                        // line(20, 13, 40, 80, &mut buffer, stride, 255, 0, 0);
                        // line(80, 40, 13, 20, &mut buffer, stride, 255, 0, 0);
                        // line(
                        //     0,
                        //     0,
                        //     1023,
                        //     767,
                        //     &mut buffer,
                        //     stride,
                        //     scale as i32,
                        //     255,
                        //     0,
                        //     0,
                        // );

                        buffer.present().unwrap();
                    }
                }
                // Event::NewEvents(StartCause::Init) => {
                //     window = Some(
                //         WindowBuilder::new()
                //             .with_title("Tiny Renderer")
                //             .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
                //             .build(&event_loop)
                //             .unwrap(),
                //     );
                //     let context = unsafe { Context::new(window.as_ref().unwrap()) }.unwrap();
                //     surface =
                //         Some(unsafe { Surface::new(&context, &window.as_ref().unwrap()) }.unwrap());
                // }
                Event::WindowEvent {
                    event:
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        },
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                // Event::MainEventsCleared => {
                //     window.as_ref().unwrap().request_redraw();
                // }
                _ => (),
            }
        })
        .unwrap();
}
