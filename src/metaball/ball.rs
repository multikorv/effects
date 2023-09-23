use crate::common::{vector::Vec2, color::Color};

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub position: Vec2,
    pub radius: f64,
    pub color: Color
}

impl Ball {
    pub fn new (position: Vec2, radius: f64, color: Color) -> Ball {
        Ball {
            position,
            radius,
            color
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn edge_pos_for_angle(&self, theta: f64) -> Vec2 {
        Vec2::new(
            self.position.x + self.radius as f64 * f64::cos(theta * (std::f64::consts::PI / 180.0)),
            self.position.y + self.radius as f64 * f64::sin(theta * (std::f64::consts::PI / 180.0)),
        )
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            position: Default::default(),
            radius: 10.0,
            color: Default::default()
        }
    }
}