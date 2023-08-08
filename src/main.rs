use winit::{
    window::WindowBuilder,
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

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    event_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            },
            _ => ()
        }
    });
}