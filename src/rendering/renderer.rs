use crate::common::color::Color;
use crate::metaball::ball::Ball;
use crate::common::vector::{Vec2, self};

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
        let width:f64 = self.width.into();
        let height:f64 = self.height.into();
        let ball: Ball = Ball::new (
            Vec2::new(width/2.0, height/2.0),
            200,
            Default::default()
        );

        let mut t1: f64 = (ball.radius) as f64;
        let mut x: f64 = ball.radius as f64;
        let mut y: f64 = 0.0;

        const SECTOR_INTERVAL_SIZE: i32 = 5;
        let mut sector_render_counter = 0;
        while x >= y {
            // Emulate a dotted circle
            if sector_render_counter % SECTOR_INTERVAL_SIZE == 0 {
                for pixel in Renderer::get_mirror_points_for_octant(&Vec2::new(x, y)) {
                    const INCREMENT: f64 = 2.0;
                    let mut trace_offset: f64 = 0.0;

                    while trace_offset <= ball.radius.into() {
                        let nearness_to_circle_edge = trace_offset/(ball.radius as f64);
                        let interpolated = vector::lerp(&(&pixel + &ball.position), &ball.position, nearness_to_circle_edge);
                        self.write_color(
                            interpolated.x.round() as u32,
                            interpolated.y.round() as u32,
                            &ball.color
                        );

                        trace_offset += INCREMENT
                    }
                }
            }

            sector_render_counter += 1;

            y += 1.0;
            t1 += y;
            let t2: f64 = t1 - x;
            if t2 >= 0.0 {
                t1 = t2;
                x -= 1.0;
            }
        }
    }

    fn get_mirror_points_for_octant(point: &Vec2) -> Vec<Vec2> {
        // TODO: Since size is known here, replace with standard array maybe?
        let mirror_points = vec![
            Vec2::new(point.x, point.y),
            Vec2::new(point.x, -point.y),
            Vec2::new(-point.x, point.y),
            Vec2::new(-point.x, -point.y),
            Vec2::new(point.y, point.x),
            Vec2::new(point.y, -point.x),
            Vec2::new(-point.y, point.x),
            Vec2::new(-point.y, -point.x)
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