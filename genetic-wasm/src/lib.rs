mod utils;

use rand::Rng;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, genetic-wasm!");
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen]
pub struct Sim {
    foo: Vec<u8>,
}

#[wasm_bindgen]
impl Sim {
    pub fn new() -> Self {
        //let mut points: Vec<Vec3> = Vec::with_capacity(n_points);
        //let mut rng = rand::thread_rng();

        //for p in &mut points {
            //p.x = (rng.gen::<f32>() - 0.5) * 40.0;
            //p.y = (rng.gen::<f32>() - 0.5) * 40.0;
            //p.z = (rng.gen::<f32>() - 0.5) * 40.0;
        //}

        
        return Self {
            foo: vec![1, 2, 3, 4, 5],
            //points,
        }
    }

    pub fn foo_ptr(&self) -> *const u8 {
        self.foo.as_ptr()
    }
}
