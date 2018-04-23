use std::sync::RwLock;

mod player_state;
use player_state::PlayerState;

extern "C" {
    fn draw_player(x: i32, y: i32);
    fn clear_screen();
}

pub struct Game {
    pub player: PlayerState,
}

impl Game {
    fn new() -> Self {
        return Game {
            player: PlayerState::new(),
        }
    }
}

let GAME = RwLock::new(Game::new());

#[no_mangle]
pub fn toggle_move_up(is_pressed: i32) {
    let mut game = GAME.write().unwrap();
    if !game.player.move_up && is_pressed {
        game.player.toggle_up();
    }
}

#[no_mangle]
pub fn toggle_move_down(is_pressed: i32) {
    let mut game = GAME.write().unwrap();
    if !game.player.move_down && is_pressed {
        game.player.toggle_down();
    }
}

#[no_mangle]
pub fn toggle_move_left(is_pressed: i32) {
    let mut game = GAME.write().unwrap();
    if !game.player.move_left && is_pressed {
        game.player.toggle_left();
    }
}

#[no_mangle]
pub fn toggle_move_right(is_pressed: i32) {
    let mut game = GAME.write().unwrap();
    if !game.player.move_right && is_pressed {
        game.player.toggle_right();
    }
}

#[no_mangle]
pub fn update(dt: f32) {
    let mut game = GAME.write().unwrap();
    game.player.update(dt)
}

#[no_mangle]
pub fn render(_: f32) {
    let game = GAME.read().unwrap();
    unsafe {
        clear_screen();
        draw_player(10, 10);
    }
}
