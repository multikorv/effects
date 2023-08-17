#[derive(Debug, Default)]
pub struct Ball {
    position: Vec3,
    radius: u32,
    color: Color
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