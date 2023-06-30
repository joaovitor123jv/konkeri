use sdl2::rect::Rect as SdlRect;

use crate::point::Point;

#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
    pub point: Point
}


impl Rect {
    #[allow(dead_code)]
    pub fn zeroed() -> Self {
        Self {
            point: Point::zeroed(),
            width: 0,
            height: 0
        }
    }

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            point: Point::new(x, y),
            width,
            height
        }
    }

    // inclusive
    #[allow(dead_code)]
    pub fn contains(&self, point: &Point) -> bool {
        let x_conditions = (point.x <= self.point.x + (self.width as i32)) && (point.x >= self.point.x);
        let y_conditions = (point.y <= self.point.y + (self.height as i32)) && (point.y >= self.point.y);
        x_conditions && y_conditions
    }

    #[allow(dead_code)]
    pub fn contains_xy(&self, x: i32, y: i32) -> bool {
        let x_conditions = (x <= self.point.x + (self.width as i32)) && (x >= self.point.x);
        let y_conditions = (y <= self.point.y + (self.height as i32)) && (y >= self.point.y);
        x_conditions && y_conditions
    }

    #[allow(dead_code)]
    pub fn centralize_vertical(&mut self, area: &Rect) {
        self.point.y = (area.point.y + (area.height as i32 / 2)) - (self.height as i32 / 2);
    }

    #[allow(dead_code)]
    pub fn centralize_horizontal(&mut self, area: &Rect) {
        self.point.x = (area.point.x + (area.width as i32 / 2)) - (self.width as i32 / 2);
    }

    #[allow(dead_code)]
    pub fn centralize(&mut self, area: &Rect) {
        self.centralize_vertical(area);
        self.centralize_horizontal(area);
    }

    pub fn to_sdl2(&self) -> SdlRect {
        SdlRect::new(self.point.x, self.point.y, self.width, self.height)
    }
}

