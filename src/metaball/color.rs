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

const WHITE: Color = Color {
    red:255,
    green: 255,
    blue: 255
};

const BLACK: Color = Color {
    red:0,
    green: 0,
    blue: 0
};

const RED: Color = Color {
    red:255,
    green: 0,
    blue: 0
};

const GREEN: Color = Color {
    red: 0,
    green: 255,
    blue: 0
};

const BLUE: Color = Color {
    red:0,
    green: 0,
    blue: 255
};