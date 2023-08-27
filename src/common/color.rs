#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (color.blue as u32) | (color.green as u32) << 8 | (color.red as u32) << 16
    }
}

impl From<&Color> for u32 {
    fn from(val: &Color) -> Self {
        (val.blue as u32) | (val.green as u32) << 8 | (val.red as u32) << 16
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 255,
            green: Default::default(),
            blue: Default::default()
        }
    }
}

pub const WHITE: Color = Color {
    red: 255,
    green: 255,
    blue: 255
};

pub const BLACK: Color = Color {
    red: 0,
    green: 0,
    blue: 0
};

pub const RED: Color = Color {
    red:255,
    green: 0,
    blue: 0
};

pub const GREEN: Color = Color {
    red: 0,
    green: 255,
    blue: 0
};

pub const BLUE: Color = Color {
    red:0,
    green: 0,
    blue: 255
};