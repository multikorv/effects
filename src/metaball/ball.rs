use crate::common::{vector::Vec3, color::Color};

#[derive(Debug)]
pub struct Ball {
    pub position: Vec3,
    pub radius: u32,
    pub color: Color
}

impl Ball {
    pub fn new (position: Vec3, radius: u32, color: Color) -> Ball {
        Ball {
            position,
            radius,
            color
        }
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