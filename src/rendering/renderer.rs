use crate::common::color::Color;
use crate::metaball::ball::Ball;
use crate::common::vector::Vec3;

use std::num::NonZeroU32;
use softbuffer::Surface;

pub struct Renderer {
    surface: Surface,
    width: u32,
    height: u32
}

// Should probably split into something threaded or yield to a certain frame count
impl Renderer {
    pub fn new(surface: Surface) -> Renderer {
        Renderer{
            surface,
            width: 0,
            height: 0,
        }
    }

    pub fn write(&mut self) {
        // TODO: Add actual meta ball rendering
        //self.debug_write();
        self.metaballs_write();
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

    // TODO: Messy casting, read up on and/or fix with assumptions
    fn metaballs_write(&mut self) {
        let ball: Ball = Ball::new (
            Vec3::new((self.width/2) as i32, (self.height/2) as i32, 0),
            100,
            Default::default()
        );

        let mut t1: i32 = ball.radius as i32;
        let mut x: i32 = ball.radius as i32;
        let mut y: i32 = 0;

        while x >= y {
            for pixel in Renderer::get_mirror_points_for_octant(&Vec3::new(x, y, 0)) {
                self.write_color(
                    (pixel.x + ball.position.x) as u32,
                    (pixel.y + ball.position.y) as u32,
                    &ball.color
              );
            }

            y += 1;
            t1 += y;
            let t2: i32 = t1 - x;
            if t2 >= 0 {
                t1 = t2;
                x -= 1;
            }
        }
    }

    fn get_mirror_points_for_octant(point: &Vec3) -> Vec<Vec3> {
        // TODO: Since size is known here, replace with standard array maybe?
        let mirror_points = vec![
            Vec3::new(point.x, point.y, 0),
            Vec3::new(point.x, -point.y, 0),
            Vec3::new(-point.x, point.y, 0),
            Vec3::new(-point.x, -point.y, 0),
            Vec3::new(point.y, point.x, 0),
            Vec3::new(point.y, -point.x, 0),
            Vec3::new(-point.y, point.x, 0),
            Vec3::new(-point.y, -point.x, 0)
        ];
        mirror_points
    }

    fn write_color(&mut self, x: u32, y: u32, color: &Color) {
        let index = self.convert_coordinates_to_buffer_index(x, y);

        let mut buffer = self.surface
            .buffer_mut()
            .expect("Could not get mutable surface buffer");

        buffer[index] = color.into();
    }

    fn convert_coordinates_to_buffer_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn debug_write(&mut self) {
        for index in 0..(self.width * self.height) {
            let x = index % self.width;
            let y = index / self.width;

            self.write_color(x, y, &Renderer::debug_color_for(x, y));
        }
    }

    fn debug_color_for(x: u32, y: u32) -> Color {
        let red: u8 = (x % 255) as u8;
        let green = (y % 255) as u8;
        let blue = (x + y % 255) as u8;

        Color::new(red, green, blue)
    }

}