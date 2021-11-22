use wasm_bindgen::{memory, prelude::*};
use js_sys::Float32Array;
use utils::set_panic_hook;
pub use sim::*;

use abi_macro::generate_typescript;

generate_typescript!();

#[macro_use]
mod utils;
mod gravity;
mod points;
mod gen;
mod sim;
mod physics;

mod memory;

/*
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/

#[wasm_bindgen]
pub fn init(opts: &JsValue) -> Sim {
    set_panic_hook();
    let opts: Opts = opts.into_serde().unwrap();

    Sim::new(opts)
}

const test_arr: &'static [f32] = &[1.0, 2.0, 3.0, 4.0, 5.0];

#[wasm_bindgen]
pub fn test() -> Float32Array {
    let ptr = test_arr.as_ptr();
    let offset = ptr as u32;
    Float32Array::new_with_byte_offset_and_length(&memory(), offset, 5)
}
