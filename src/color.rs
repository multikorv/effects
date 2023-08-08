pub struct Color {
    red: u32,
    green: u32,
    blue: u32
}

impl Color {
    pub fn new(red:u32, green:u32, blue:u32) -> Color {
        Color { red, green, blue }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color.blue | color.green << 8 | color.red << 16
    }
}