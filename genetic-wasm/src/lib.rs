use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
use serde::Serialize;
pub use sim::*;

#[macro_use]
mod utils;
mod gravity;
mod points;
mod gen;
mod sim;
mod physics;
mod memory;

pub use memory::*;

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

#[wasm_bindgen]
pub struct TestMyPtr {
    data: Vec<f32>,
}

#[wasm_bindgen]
impl TestMyPtr {
    pub fn new() -> Self {
        Self {
            data: (1u8..10).map(f32::from).collect()
        }
    }

    pub fn my_version(&self) -> JsValue {
        let my_ptr = BufferF32Ptr::from(&self.data);
        JsValue::from_serde(&my_ptr).unwrap()
    }

    pub fn their_version(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }
}
