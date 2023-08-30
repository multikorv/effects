mod metaball;
mod common;
mod rendering;

use common::time::Time;
use metaball::state::State;
use rendering::renderer::Renderer;

use softbuffer::{
    Context,
    Surface,
};

use winit::{
    window::{
        WindowBuilder,
        Window
    },
    event_loop::{
        EventLoop,
        ControlFlow
    },
    event::{
        Event,
        WindowEvent
    }
};

fn main() {
    let event_loop: EventLoop<()> = EventLoop::new();

    let window: Window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let context: Context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let surface: Surface = unsafe { softbuffer::Surface::new(&context, &window).unwrap() };
    let mut renderer = Renderer::new(surface);
    let (width, height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };

    let mut time = Time::new();

    let mut ball_state = State::new();

    renderer.resize(width, height);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                renderer.resize(width, height);

                time.tick();
                ball_state.tick(&time);

                renderer.metaballs_write(&ball_state, &time);

                renderer.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => {}
        }
    });
}