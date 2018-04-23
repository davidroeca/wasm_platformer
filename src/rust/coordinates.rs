pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x, y }
    }
}
