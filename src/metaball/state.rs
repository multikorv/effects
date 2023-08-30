use crate::{
    metaball::ball::Ball,
    common::{
        vector::Vec2,
        color::Color,
        time::Time
    }
};

use rand::Rng;

pub struct State {
    pub balls: Vec<Ball>,
    pub world_center: Vec2
}

impl State {
    pub fn new(world_center: Vec2) -> Self {
        State {
            balls: vec![
                Ball::new(Vec2::new(0.0, 0.0), 100, Color::new(255, 220, 53)),
                Ball::new(Vec2::new(0.0, 0.0), 90, Color::new(198, 210, 61)),
                Ball::new(Vec2::new(0.0, 0.0), 80, Color::new(131, 170, 63)),
                Ball::new(Vec2::new(0.0, 0.0), 70, Color::new(93, 136, 66)),
                Ball::new(Vec2::new(0.0, 0.0), 60, Color::new(132, 79, 27)),
                Ball::new(Vec2::new(0.0, 0.0), 60, Color::new(255, 220, 53)),
                Ball::new(Vec2::new(0.0, 0.0), 70, Color::new(198, 210, 61)),
                Ball::new(Vec2::new(0.0, 0.0), 80, Color::new(131, 170, 63)),
                Ball::new(Vec2::new(0.0, 0.0), 90, Color::new(93, 136, 66)),
                Ball::new(Vec2::new(0.0, 0.0), 100, Color::new(132, 79, 27)),
            ],
            world_center
        }
    }

    pub fn tick(&mut self, time: &Time) {
        // TODO: Fix transforms instead

        let mut count = 1.0;
        let mut rng = rand::thread_rng();
        self.balls
            .iter_mut()
            .for_each(|ball| {
                let time_passed_seconds = time.elapsed.as_secs_f64();
                let amp_factor = 1.0 + (count * 0.1);
                let time_factor = 1.0 + (count * 0.1);
                ball.position.x = self.world_center.x + 100.0 * amp_factor * f64::cos(time_passed_seconds * time_factor);
                ball.position.y = self.world_center.y + 100.0 * amp_factor * f64::sin(time_passed_seconds * time_factor);
                count += 1.0;
            });
    }
}
