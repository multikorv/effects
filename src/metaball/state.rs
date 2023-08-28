use crate::{
    metaball::ball::Ball,
    common::{
        vector::Vec2,
        color::{
            self, Color
        },
        time::Time
    }
};

pub struct State {
    pub balls: Vec<Ball>,
}

impl State {
    pub fn new() -> Self {
        State {
            balls: vec![
                Ball::new(Vec2::new(300.0, 300.0), 100, Color::new(52, 164, 235)),
                Ball::new(Vec2::new(500.0, 200.0), 90, Color::new(135, 64, 38))
            ]
        }
    }

    pub fn tick(&mut self, time: &Time) {
        // TODO: Fix transforms instead of this manual update of original_balls mess
        // and potentially use stacks for transforms applied to object

        let mut count = 0;
        self.balls
            .iter_mut()
            .for_each(|ball| {
                let time_passed_seconds = time.elapsed.as_secs_f64();
                if count == 0
                {
                    ball.position.x = 200.0 + 30.0 * f64::sin(time_passed_seconds * 2.0);
                    ball.position.y = 300.0 + 50.0 * f64::cos(time_passed_seconds / 2.0);
                    count += 1;
                }
                else {
                    ball.position.x = 500.0 + 30.0 * f64::sin(time_passed_seconds * 2.3) + 70.0 * f64::cos(time_passed_seconds * 3.0);
                    ball.position.y = 400.0 + 90.0 * f64::cos(time_passed_seconds / 2.0) + 60.0 * f64::sin(time_passed_seconds * 1.3);
                    count = 0;
                }
            }
        );
    }
}
