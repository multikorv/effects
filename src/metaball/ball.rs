use crate::common::{vector::Vec2, color::Color};

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub position: Vec2,
    pub radius: u32,
    pub color: Color
}

impl Ball {
    pub fn new (position: Vec2, radius: u32, color: Color) -> Ball {
        Ball {
            position,
            radius,
            color
        }
    }

    pub fn diameter(&self) -> u32 {
        self.radius * 2
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            position: Default::default(),
            radius: 10,
            color: Default::default()
        }
    }
}