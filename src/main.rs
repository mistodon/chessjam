extern crate chessjam;
extern crate glium;

mod input;

use glium::Display;
use glium::glutin::EventsLoop;

use input::*;


fn main() {
    use glium::glutin::{Api, ContextBuilder, GlProfile, GlRequest, WindowBuilder};

    let events_loop = &mut EventsLoop::new();

    let window = WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_title("Purchess");

    let context = ContextBuilder::new()
        .with_depth_buffer(24)
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 0)))
        .with_multisampling(4)
        .with_vsync(true);

    let display = &Display::new(window, context, &events_loop).unwrap();

    loop {
        let rerun = run_game(display, events_loop);

        if !rerun {
            break;
        }
    }
}


fn run_game(display: &Display, events_loop: &mut EventsLoop) -> bool {
    use std::time::Instant;

    let mut frame_time = Instant::now();
    let mut keyboard = KeyboardState::default();

    loop {
        let (dt, now) = chessjam::delta_time(frame_time);
        frame_time = now;

        // handle_events
        let mut quitting = false;
        {
            use glium::glutin::{ElementState, Event, WindowEvent};

            keyboard.begin_frame();

            events_loop.poll_events(|event| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        let pressed = input.state == ElementState::Pressed;

                        if let Some(key) = input.virtual_keycode {
                            if pressed {
                                keyboard.press(key, input.modifiers);
                            }
                            else {
                                keyboard.release(key, input.modifiers);
                            }
                        }
                    }
                    WindowEvent::Closed => quitting = true,
                    _ => (),
                },
                _ => (),
            });

            if keyboard.pressed(Key::Escape) {
                quitting = true;
            }
        }

        if quitting {
            return false;
        }

        // render
        {
            use glium::{Rect, Surface};

            let mut frame = display.draw();
            frame.clear_color_srgb(0.4, 0.0, 0.6, 1.0);

            const TARGET_ASPECT: f32 = 16.0 / 9.0;
            let viewport = {
                let (left, bottom, width, height) = chessjam::viewport_rect(
                    display.get_framebuffer_dimensions(),
                    TARGET_ASPECT,
                );
                Rect {
                    left,
                    bottom,
                    width,
                    height,
                }
            };

            frame.clear(
                Some(&viewport),
                Some((0.6, 1.0, 0.4, 1.0)),
                true,
                None,
                None,
            );

            frame.finish().unwrap();
        }
    }
}
