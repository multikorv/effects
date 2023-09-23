#[cfg(test)]
mod tests;

use std::ops::{Sub, Add, Mul};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
}

// TODO: Investigate this.
// Do I have to duplicate simple arithmetics if the instance is borrowed?
// In turn, this would lead to duplicated tests as well?
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        rhs * self
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<&Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}

impl Eq for Vec2 {}

// TODO: Parameter checking
pub fn lerp(p1: &Vec2, p2: &Vec2, t: f64) -> Vec2 {
    (1.0 - t) * p1 + t * p2
}

pub fn distance(p1: &Vec2, p2: &Vec2) -> f64 {
    (f64::powi(p2.x - p1.x, 2) + f64::powi(p2.y - p1.y, 2)).sqrt()
}