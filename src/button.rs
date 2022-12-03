use crate::color::Color;
use crate::rect::Rect;
use sdl2;

enum RectState {
    IDLE,
    HOVER,
    PRESSED
}

pub struct Button {
    pub rect: Rect,
    default_color: Color,
    hover_color: Color,
    pressed_color: Color,
    #[allow(dead_code)]
    text: String,
    state: RectState
}

impl Button {
    pub fn new(text: &str) -> Self {
        Button {
            rect: Rect::zeroed(),
            default_color: Color::black(),
            hover_color: Color::white(),
            pressed_color: Color::red(),
            text: text.to_string(),
            state: RectState::IDLE
        }
    }

    pub fn update(&mut self, x: i32, y: i32, is_clicking: bool) {
        if !self.rect.contains_xy(x, y) {
            self.state = RectState::IDLE;
            return;
        }

        self.state = match is_clicking { true => RectState::PRESSED, false => RectState::HOVER };
    }

    pub fn render(&self, canvas: &mut sdl2::render::WindowCanvas) {
        let current_color: &Color = match self.state {
            RectState::HOVER => &self.hover_color,
            RectState::IDLE => &self.default_color,
            RectState::PRESSED => &self.pressed_color 
        };

        canvas.set_draw_color(current_color.to_sdl2());
        let _ = canvas.fill_rect(self.rect.to_sdl2());
    }
}

