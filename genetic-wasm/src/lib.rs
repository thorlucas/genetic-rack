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
    points: Vec<Vec3>,
}

#[wasm_bindgen]
impl Sim {
    pub fn new(n_points: usize) -> Self {
        let mut points: Vec<Vec3> = Vec::with_capacity(n_points);
        let mut rng = rand::thread_rng();

        for i in 0..n_points {
            points.push(Vec3 {
                x: rng.gen_range(-20.0..20.0),
                y: rng.gen_range(-20.0..20.0),
                z: rng.gen_range(-20.0..20.0),
            });
        }
        
        return Self {
            points,
        }
    }

    pub fn points_buffer_ptr(&self) -> *const f32 {
        self.points.as_ptr() as *const f32
    }
}
