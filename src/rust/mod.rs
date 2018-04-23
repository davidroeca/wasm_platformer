use std::sync::RwLock;

mod coordinates;
mod player_state;

use rust::player_state::PlayerState;

extern "C" {
    fn draw_player(x: f32, y: f32);
    fn clear_screen();
}

pub struct WindowState {
    width: i32,
    height: i32,
}

impl WindowState {
    pub fn new(width: i32, height: i32) -> Self {
        WindowState {
            width,
            height,
        }
    }
}

pub struct Game {
    pub player: PlayerState,
    pub window: WindowState,
}

impl Game {
    fn new() -> Self {
        Game {
            player: PlayerState::new(),
            window: WindowState::new(400, 275),
        }
    }
}


#[no_mangle]
pub extern fn create_game() -> *mut RwLock<Game> {
    Box::into_raw(Box::new(RwLock::new(Game::new())))
}

#[no_mangle]
pub extern fn free_game(ptr: *mut RwLock<Game>) {
    if ptr.is_null() {
        return
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern fn toggle_move_up(ptr: *const RwLock<Game>, is_down: i32) {
    let lock = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let mut game = lock.write().unwrap();
    game.player.toggle_up(is_down != 0);
}

#[no_mangle]
pub extern fn toggle_move_down(ptr: *const RwLock<Game>, is_down: i32) {
    let lock = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let mut game = lock.write().unwrap();
    game.player.toggle_down(is_down != 0);
}

#[no_mangle]
pub extern fn toggle_move_left(ptr: *const RwLock<Game>, is_down: i32) {
    let lock = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let mut game = lock.write().unwrap();
    game.player.toggle_left(is_down != 0);
}

#[no_mangle]
pub extern fn toggle_move_right(ptr: *const RwLock<Game>, is_down: i32) {
    let lock = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let mut game = lock.write().unwrap();
    game.player.toggle_right(is_down != 0);
}

#[no_mangle]
pub extern fn update(ptr: *const RwLock<Game>, dt: f32) {
    let lock = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let mut game = lock.write().unwrap();
    game.player.update(dt);
}

#[no_mangle]
pub extern fn render(ptr: *const RwLock<Game>, _: f32) {
    let lock = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let game = lock.read().unwrap();
    let x = game.player.position.x;
    let y = game.player.position.y;
    unsafe {
        clear_screen();
        draw_player(x, y);
    }
}
