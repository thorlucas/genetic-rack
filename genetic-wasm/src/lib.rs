use wasm_bindgen::prelude::*;

mod utils;
mod sim;
mod points;
mod gen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//pub use sim::{Sim, SimBuilder};
pub use sim::Sim;
pub use gen::*;
use utils::set_panic_hook;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
}
