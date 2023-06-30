use crate::color::Color;
use crate::rect::Rect;


pub struct RenderableRect {
    pub rect: Rect,
    pub color: Color
}

impl RenderableRect {
    pub fn new(color: Color, x: i32, y: i32, width: u32, height: u32) -> RenderableRect {
        RenderableRect {
            color,
            rect: Rect::new(x, y, width, height)
        }
    }
    
    #[allow(dead_code)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn render(&self, canvas: &mut sdl2::render::WindowCanvas) {
        canvas.set_draw_color(self.color.to_sdl2());
        let _ = canvas.fill_rect(self.rect.to_sdl2());
    }

    #[allow(dead_code)]
    pub fn centralize_vertical(&mut self, area: &Rect) {
        self.rect.centralize_vertical(area);
    }

    #[allow(dead_code)]
    pub fn centralize_horizontal(&mut self, area: &Rect) {
        self.rect.centralize_horizontal(area);
    }

    #[allow(dead_code)]
    pub fn centralize(&mut self, area: &Rect) {
        self.rect.centralize(area);
    }

}
