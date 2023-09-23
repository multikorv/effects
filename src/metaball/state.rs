use crate::{
    metaball::ball::Ball,
    common::{
        vector::Vec2,
        color::Color,
        time::Time
    }
};

pub struct State {
    pub balls: Vec<Ball>,
    pub world_center: Vec2
}

impl State {
    pub fn new(world_center: Vec2) -> Self {
        State {
            balls: vec![
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(25, 220, 63)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(198, 40, 161)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(200, 10, 66)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(235, 215, 245)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(22, 79, 27)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(198, 21, 61)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(43, 35, 63)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(93, 36, 66)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(132, 79, 27)),
                Ball::new(Vec2::new(0.0, 0.0), 100.0, Color::new(25, 42, 93)),
            ],
            world_center
        }
    }

    pub fn tick(&mut self, time: &Time) {
        self.colorful_spiraly(time);
    }

    fn colorful_spiraly(&mut self, time: &Time) {
        let mut count = 1.0;
        self.balls
            .iter_mut()
            .for_each(|ball| {
                let time_passed_seconds = time.elapsed.as_secs_f64();
                let amp = 100.0 * (1.0 + count * 0.1);
                let speed = 1.0 + (count * 0.1);

                ball.position.x = self.world_center.x + amp * f64::cos(time_passed_seconds * speed);
                ball.position.y = self.world_center.y + amp * f64::sin(time_passed_seconds * speed);

                count += 1.0;
            });
    }
}
