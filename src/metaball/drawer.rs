use crate::metaball::color::Color;

use std::num::NonZeroU32;
use softbuffer::Surface;

pub struct Drawer {
    surface: Surface,
    width: u32,
    height: u32
}

// Should probably split into something threaded or yield to a certain frame count
// TODO: Investigate if i can avoid static in this module
impl Drawer {
    pub fn new(surface: Surface) -> Drawer {
        Drawer{
            surface,
            width: 0,
            height: 0,
        }
    }

    // TODO: Fix width and height set, should be set elsewhere
    pub fn write(&mut self) {

        for index in 0..(self.width * self.height) {
            // TODO: Add actual meta ball rendering
            let x = index %  self.width;
            let y = index / self.width;

            let red = x % 255;
            let green = y % 255;
            let blue = x + y % 255;

            let mut buffer = self.surface
                .buffer_mut()
                .expect("Could not get mutable surface buffer");

            buffer[index as usize] = Color::new(red, green, blue).into();
        }
    }

    pub fn present(&mut self) {
        let buffer = self.surface
            .buffer_mut()
            .expect("Could not get mutable surface buffer");
        buffer.present().expect("Failed to presest buffer");
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // TODO: Fix a vector for this?
        self.width = width;
        self.height = height;
        self.surface.resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap()
        )
        .unwrap();
    }
}