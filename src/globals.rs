use crate::rect::Rect;

const ZOOM_STEP: f32 = 0.25;

pub struct Global {
    pub window_width: u32,
    pub window_height: u32,
    pub fps_amount: u32,
    pub viewport: Rect,
    pub zoom: f32,
}

impl Global {
    pub fn new() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            // viewport: Rect::new(250, 550, 1050, 800),
            viewport: Rect::new(0, 0, 1050, 800),
            zoom: 1.0,
            // fps_amount: 60
            fps_amount: 75
        }
    }

    pub fn apply_zoom(&mut self, zoom_direction: i32) {
        if zoom_direction == 1 {
            self.zoom += ZOOM_STEP;

            if self.zoom > 10.0 {
                self.zoom = 10.0;
            }
        } else if zoom_direction == -1 {
            self.zoom -= ZOOM_STEP;

            if self.zoom < 0.25 {
                self.zoom = 0.25;
            }
        }
    }

    pub fn apply_offset(&mut self, x_rel: i32, y_rel: i32) {
        self.viewport.point.x -= x_rel;
        self.viewport.point.y -= y_rel;
    }

    pub fn update_window_dimensions(&mut self, width: u32, height: u32) {
        let width_diff =  (width as i32) - (self.window_width as i32);
        let height_diff =  (height as i32) - (self.window_height as i32);

        self.window_width = width;
        self.window_height = height;

        self.viewport.width = (width_diff + (self.viewport.width as i32)) as u32;
        self.viewport.height = (height_diff + (self.viewport.height as i32)) as u32;
    }
}
