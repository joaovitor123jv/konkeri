
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn zeroed() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}
