use sdl2::pixels::Color as SdlColor;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    #[allow(dead_code)]
    alpha: u8
}

impl Color {
    #[allow(dead_code)]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue, alpha: 0 }
    }

    #[allow(dead_code)]
    pub fn red() -> Self {
        Color::new(255, 0, 0)
    }

    #[allow(dead_code)]
    pub fn green() -> Self {
        Color::new(0, 255, 0)
    }

    #[allow(dead_code)]
    pub fn blue() -> Self {
        Color::new(0, 0, 255)
    }

    #[allow(dead_code)]
    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    #[allow(dead_code)]
    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub fn to_sdl2(&self) -> SdlColor {
        SdlColor::RGB(self.red, self.green, self.blue)
    }
}
