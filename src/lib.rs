///
/// This file defines the final interface of all rust code, under `rust`.
/// There should be very little code here, only `pub use` statements.
///
mod rust;

pub use rust::{
    render,
    update,
};
