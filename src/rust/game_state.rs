static const INV_ROOT_2: f32 = 1.0 / 2.0.sqrt()

struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    fn new(x: i32, y: i32) -> Self {
        return Velocity{ x, y }
    }
}

struct GameState {
    move_up: bool,
    move_down: bool,
    move_right: bool,
    move_left: bool,
    speed: f32,
}


impl GameState {
    fn new() -> Self {
        return GameState {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            speed: 10,
        }
    }

    fn velocity(&self) -> Velocity {
        let y_contrib = 1.0 * (move_down as f32) - 1.0 * (move_up as f32)
        let x_contrib = 1.0 * (move_down as f32) - 1.0 * (move_up as f32)
        if (y_contrib && x_contrib) {
            Velocity::new(INV_ROOT_2 * x_contrib, INV_ROOT_2 * y_contrib)
        } else {
            Velocity::new(x_contrib, y_contrib)
        }
    }
}
