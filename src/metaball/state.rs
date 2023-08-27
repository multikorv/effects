use core::time;
use std::time::Instant;

use crate::{
    metaball::ball::Ball,
    common::{
        vector::Vec2,
        color::{
            self
        }
    }
};


pub struct State {
    pub balls: Vec<Ball>,
    start: Instant,
}

impl State {
    pub fn new() -> Self {
        let predefined_balls = vec![
            Ball::new(Vec2::new(300.0, 300.0), 100, color::RED),
            Ball::new(Vec2::new(500.0, 200.0), 90, color::WHITE)
        ];
        State {
            balls: predefined_balls,
            start: Instant::now()
        }
    }

    pub fn tick(&mut self) {
        // TODO: Fix transforms instead of this manual update of original_balls mess
        // and potentially use stacks for transforms applied to object
        let time_passed = Instant::now() - self.start;
        let mut count = 0;
        self.balls.iter_mut().for_each(|ball| {
            let time_passed_seconds = time_passed.as_secs_f64();
            if count == 0
            {
                ball.position.x = 20.0 + 70.0 * f64::sin(time_passed_seconds * 2.0);
                ball.position.y = 30.0 + 100.0 * f64::cos(time_passed_seconds / 2.0);
                count += 1;
            }
            else {
                ball.position.x = 50.0 + 80.0 * f64::sin(time_passed_seconds * 2.3) + 70.0 * f64::cos(time_passed_seconds * 3.0);
                ball.position.y = 40.0 + 95.0 * f64::cos(time_passed_seconds / 2.0) + 60.0 * f64::sin(time_passed_seconds * 1.3);
                count = 0;
            }
        });
    }
}
