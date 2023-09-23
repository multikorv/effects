use crate::common::color::Color;
use crate::common::time::Time;
use crate::common::vector::{
    Vec2,
    self
};
use crate::metaball::state::State;

use std::num::NonZeroU32;
use softbuffer::Surface;

pub struct Renderer {
    surface: Surface,
    width: u32,
    height: u32,
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

    pub fn metaballs_write(&mut self, state: &State, time: &Time) {
        self.clear_buffer();

        self.write_with_asymmetric_lines(state, time);
        //self.write_as_dotted_sun(state, time);
    }

    fn write_with_asymmetric_lines(&mut self, state: &State, time: &Time) {
        for ball in state.balls.iter() {
            const MAX_THETA: f64 = 180.0;
            let theta_offset: f64 = 8.0 * f64::sin(time.elapsed.as_secs_f64());
            let mut theta: f64 = theta_offset;
            let theta_increment = 2.0;

            while theta <= (MAX_THETA + theta_offset) {
                let edge = ball.edge_pos_for_angle(theta);

                const TRACE_INCREMENT: f64 = 5.0;
                let mut trace_offset: f64 = 0.0;

                let dist_to_mid = vector::distance(&edge, &Vec2::new(edge.x, ball.position.y));

                let direction = match ball.position.x - edge.x {
                    x if x < 0.0 => 1.0,
                    x if x > 0.0 => -1.0,
                    _ => 0.0
                };

                while trace_offset <= (ball.diameter()).into() {
                    let nearness_to_circle_edge = trace_offset/(ball.diameter() as f64);

                    // Mirror to other side, interpolate to that
                    let edge_on_opposite_side = Vec2::new(edge.x, edge.y - dist_to_mid*2.0);
                    let interpolated_pixel = vector::lerp(
                        &edge,
                        &edge_on_opposite_side,
                        nearness_to_circle_edge
                    );

                    let mut pixel_dist_to_midpoint = 1.0 - (vector::distance(&interpolated_pixel, &ball.position) / ball.radius as f64);
                    let mut nearness_to_mid_x = vector::distance(&interpolated_pixel, &Vec2::new(ball.position.x, interpolated_pixel.y)) / ball.radius as f64;

                    const BEND_AMP: f64 = 90.0;
                    pixel_dist_to_midpoint *= nearness_to_mid_x;
                    pixel_dist_to_midpoint *= BEND_AMP;
                    pixel_dist_to_midpoint *= direction;

                    self.write_color(
                        (interpolated_pixel.x + (pixel_dist_to_midpoint)).round() as u32,
                        interpolated_pixel.y.round() as u32,
                        &ball.color
                    );

                    trace_offset += TRACE_INCREMENT;
                }
                theta += theta_increment;
            }
        }
    }

    fn write_as_dotted_sun(&mut self, state: &State, time: &Time) {
        for ball in state.balls.iter() {
            let mut t1: f64 = ball.radius as f64;
            let mut x: f64 = ball.radius as f64;
            let mut y: f64 = 0.0;

            const SECTOR_INTERVAL_SIZE: i32 = 10;
            let mut sector_render_counter: i32 = 3;

            while x >= y {
                for pixel in Renderer::get_mirror_points_for_octant(&Vec2::new(x, y)) {
                    if sector_render_counter % SECTOR_INTERVAL_SIZE == 0 {
                        const INCREMENT: f64 = 2.0;
                        let mut trace_offset: f64 = 0.0;

                        const SEGMENT_INTERVAL_SIZE: i32 = 2;

                        let offset_pixel = pixel;
                        let transformed_pixel_location = offset_pixel + ball.position;

                        while trace_offset <= ball.radius.into() {
                            let nearness_to_circle_edge = trace_offset/(ball.radius as f64);

                            let interpolated = vector::lerp(&transformed_pixel_location, &Vec2::new(transformed_pixel_location.x, ball.position.y), nearness_to_circle_edge);

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

        // Cull anything outside of the buffer, since that will not appear on screen
        if index > self.get_buffer_max_index() { return };

        let mut buffer = self.get_buffer();

        buffer[index] = color.into();
    }

    fn get_buffer_max_index(&mut self) -> usize {
        self.get_buffer_length() - 1
    }

    fn get_buffer_length(&mut self) -> usize {
        self.get_buffer().len()
    }

    fn convert_coordinates_to_buffer_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn get_buffer(&mut self) -> softbuffer::Buffer {
        self.surface
            .buffer_mut()
            .expect("Could not get mutable surface buffer")
    }

    fn clear_buffer(&mut self) {
        self.get_buffer()
            .iter_mut()
            .for_each(|b| *b = 0);
    }
}
