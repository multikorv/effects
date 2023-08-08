use std::num::NonZeroU32;

use softbuffer::{
    Context,
    Surface, Buffer
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
    let mut surface: Surface = unsafe { softbuffer::Surface::new(&context, &window).unwrap() };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface.resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap()
                )
                .unwrap();

                let mut buffer : Buffer <'_> = surface
                    .buffer_mut()
                    .expect("Could not get mutable surface buffer");

                for index in 0..(width * height) {
                    let x = index % width;
                    let y = index / width;

                    let red = x % 255;
                    let green = y % 255;
                    let blue = x + y % 255;

                    buffer[index as usize] = Color::new(red, green, blue).as_pixel();
                }

                buffer.present().expect("Failed to present surface buffer");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit
            },
            _ => ()
        }
    });
}

pub struct Color {
    red: u32,
    green: u32,
    blue: u32
}

impl Color {
    pub fn new(red:u32, green:u32, blue:u32) -> Color {
        Color { red, green, blue }
    }

    /// pixel format as defined by Buffer in softbuffer
    pub fn as_pixel(self) -> u32 {
        self.blue | self.green << 8 | self.red << 16
    }
}