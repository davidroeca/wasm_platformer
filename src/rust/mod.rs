mod game_state;
use game_state::GameState;

extern "C" {
    fn draw_player(x: i32, y: i32);
    fn clear_screen();
}

static mut ref const GAME_STATE = GameState::new();

#[no_mangle]
pub fn toggle_move_up(is_pressed: i32) {
    GAME_STATE.
}

#[no_mangle]
pub fn toggle_move_down(is_pressed: i32) {}

#[no_mangle]
pub fn toggle_move_right(is_pressed: i32) {}

#[no_mangle]
pub fn toggle_move_left(is_pressed: i32) {}

#[no_mangle]
pub fn render(_: f32) {
    unsafe {
        clear_screen();
        draw_player(10, 10);
    }
}

#[no_mangle]
pub fn update(_: f32) {
}

