use crate::metaball::color::Color;

use std::num::NonZeroU32;
use softbuffer::Surface;

pub struct Drawer {
    surface: Surface,
    width: u32,
    height: u32
}

// Should probably split into something threaded or yield to a certain frame count
impl Drawer {
    pub fn new(surface: Surface) -> Drawer {
        Drawer{
            surface,
            width: 0,
            height: 0,
        }
    }

    pub fn write(&mut self) {
        // TODO: Add actual meta ball rendering
        self.debug_write();
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
        .expect("Could not resize surface");
    }

    fn metaballs_write(&mut self) {
        let mut buffer = self.surface
            .buffer_mut()
            .expect("Could not get mutable surface buffer");
    }

    fn debug_write(&mut self)
    {
        let mut buffer = self.surface
            .buffer_mut()
            .expect("Could not get mutable surface buffer");

        for index in 0..(self.width * self.height) {
            let x = index %  self.width;
            let y = index / self.width;

            buffer[index as usize] = Drawer::debug_color_for(x, y).into();
        }
    }

    fn debug_color_for(x: u32, y: u32) -> Color
    {
        let red: u8 = (x % 255) as u8;
        let green = (y % 255) as u8;
        let blue = (x + y % 255) as u8;

        Color::new(red, green, blue)
    }

}