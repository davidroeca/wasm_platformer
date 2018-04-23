mod coordinates;
use coordinates::Vector;

static const INV_ROOT_2: f32 = 1.0 / 2.0.sqrt();

pub struct PlayerState {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool,
    pub max_speed: f32,
    pub position: Vector,
}

impl PlayerState {
    pub fn new() -> Self {
        return PlayerState {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            max_speed: 3.0,  // player width per second
            position: Vector::new(0.0, 0.0),
        }
    }

    pub fn toggle_up(&mut self) {
        self.move_up = !self.move_up;
    }

    pub fn toggle_down(&mut self) {
        self.move_down = !self.move_down;
    }

    pub fn toggle_left(&mut self) {
        self.move_left = !self.move_left;
    }

    pub fn toggle_right(&mut self) {
        self.move_right = !self.move_right;
    }

    fn update(&mut self, dt: f32) {
        let y_contrib = self.max_speed * (
            (move_down as f32) - (move_up as f32)
        );
        let x_contrib = self.max_speed * (
            (move_down as f32) - (move_up as f32)
        );
        if x_contrib && y_contrib {
            self.position.x +=  INV_ROOT_2 * x_contrib * dt;
            self.position.y +=  INV_ROOT_2 * y_contrib * dt;
        } else {
            self.position.x +=  x_contrib * dt;
            self.position.y +=  y_contrib * dt;
        }
    }
}
