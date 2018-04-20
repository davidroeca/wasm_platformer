extern "C" {
    fn draw_player(x: f32, y: f32);
    fn clear_screen();
}

#[no_mangle]
pub fn render(_: f32) {
    unsafe {
        clear_screen();
        draw_player(10.0, 10.0);
    }
}

#[no_mangle]
pub fn update(_: f32) {}
