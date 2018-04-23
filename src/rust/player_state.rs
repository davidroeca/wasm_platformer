use rust::coordinates::Vector;

pub struct PlayerState {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool,
    pub max_speed: f32,
    pub position: Vector,
    pub width: f32,
    pub height: f32,
}

impl PlayerState {
    pub fn new() -> Self {
        return PlayerState {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            max_speed: 80.0,  // player width per second
            position: Vector::new(0.0, 0.0),
            width: 20.0,
            height: 20.0,
        }
    }

    pub fn toggle_up(&mut self, is_down: bool) {
        self.move_up = is_down;
    }

    pub fn toggle_down(&mut self, is_down: bool) {
        self.move_down = is_down;
    }

    pub fn toggle_left(&mut self, is_down: bool) {
        self.move_left = is_down;
    }

    pub fn toggle_right(&mut self, is_down: bool) {
        self.move_right = is_down;
    }

    pub fn update(&mut self, dt: f32) {
        use std::f32::consts::SQRT_2;
        let convert_bool = |b: bool| -> f32 {if b { 1.0 } else { 0.0 }};
        let y_contrib = dt * self.width * self.max_speed * (
            convert_bool(self.move_down) - convert_bool(self.move_up)
        );
        let x_contrib = dt * self.width * self.max_speed * (
            convert_bool(self.move_right) - convert_bool(self.move_left)
        );
        if x_contrib != 0.0 && y_contrib != 0.0 {
            self.position.x += x_contrib / (SQRT_2 as f32);
            self.position.y += y_contrib / (SQRT_2 as f32);
        } else {
            self.position.x +=  x_contrib;
            self.position.y +=  y_contrib;
        }
    }
}
