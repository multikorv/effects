#[cfg(test)]
mod tests;

use std::time::{Instant, Duration};

pub struct Time {
    pub delta: Duration,
    pub elapsed: Duration,
    last_update: Instant
}

// TODO: Should this be made self sustainable?
impl Time {
    pub fn new () -> Time {
        Time {
            delta: Duration::new(0, 0),
            elapsed: Duration::new(0, 0),
            last_update: Instant::now()
        }
    }

    // TODO: Maybe not that efficient to always replace duration and instant objects?
    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_update;
        self.last_update = now;
        self.elapsed += self.delta;
    }
}