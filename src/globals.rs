use crate::{cli::CliArgs, rect::Rect};

const ZOOM_STEP: f32 = 0.25;
const ZOOM_IN: i32 = 1;
const ZOOM_OUT: i32 = -1;

pub struct Global {
    pub window: Rect,
    pub fps_amount: u32,
    pub viewport: Rect,
    pub zoom: f32,
}

impl Global {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            window: Rect::new(0, 0, 800, 600),
            // viewport: Rect::new(250, 550, 1050, 800),
            viewport: Rect::new(0, 0, 1050, 800),
            zoom: 1.0,
            // fps_amount: 60
            fps_amount: 75,
        }
    }

    pub fn from_cli_args(args: CliArgs) -> Self {
        Self {
            window: Rect::new(0, 0, args.width, args.height),
            viewport: Rect::new(0, 0, args.viewport_width, args.viewport_height),
            zoom: args.zoom,
            fps_amount: args.fps,
        }
    }

    pub fn apply_zoom(&mut self, zoom_direction: i32) {
        if zoom_direction == ZOOM_IN {
            self.zoom += ZOOM_STEP;

            if self.zoom > 4.0 {
                self.zoom = 4.0;
            }
            println!("Zoom atual = {}", self.zoom);
        } else if zoom_direction == ZOOM_OUT {
            self.zoom -= ZOOM_STEP;

            if self.zoom < 1.0 {
                self.zoom = 1.0;
            }
            println!("Zoom atual = {}", self.zoom);
        }
    }

    pub fn apply_offset(&mut self, x_rel: i32, y_rel: i32) {
        self.viewport.point.x -= x_rel;
        self.viewport.point.y -= y_rel;
    }

    pub fn update_window_dimensions(&mut self, width: u32, height: u32) {
        let width_diff = (width as i32) - (self.window.width as i32);
        let height_diff = (height as i32) - (self.window.height as i32);

        self.window.width = width;
        self.window.height = height;

        self.viewport.width = (width_diff + (self.viewport.width as i32)) as u32;
        self.viewport.height = (height_diff + (self.viewport.height as i32)) as u32;
    }
}
