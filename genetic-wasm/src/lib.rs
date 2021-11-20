use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
pub use sim::*;

#[macro_use]
mod utils;
mod gravity;
mod points;
mod gen;
mod sim;
mod physics;
mod memory;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Float32InterleavedBuffer {
    pub buffer_ptr: *const f32,
    pub stride: usize,
    pub offset: usize,
    pub items: usize,
}

#[wasm_bindgen]
pub fn init(opts: &JsValue) -> Sim {
    set_panic_hook();
    let opts: Opts = opts.into_serde().unwrap();

    Sim::new(opts)
}
