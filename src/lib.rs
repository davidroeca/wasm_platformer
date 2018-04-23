///
/// This file defines the final interface of all rust code, under `rust`.
/// There should be very little code here, only `pub use` statements.
///
mod rust;

pub use rust::{
    create_game,
    free_game,
    toggle_move_up,
    toggle_move_down,
    toggle_move_left,
    toggle_move_right,
    render,
    update,
};
