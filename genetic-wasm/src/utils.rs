use wasm_bindgen::prelude::*;
use std::f32::consts::{LN_2, TAU};
use glam::{Mat3, Vec3};
use rand::Rng;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn rand_unit() -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta = (1.0 - 2.0 * rng.gen::<f32>()).acos();
    let phi = TAU * rng.gen::<f32>();
    
    let rot = Mat3::from_rotation_ypr(theta, phi, 0.0);
    rot * Vec3::Y
}

pub fn rand_lifetime(half_life: f32, max_life: Option<f32>) -> f32 {
    let lambda = LN_2 / half_life;
    let r: f32 = rand::thread_rng().gen();
    let h = -r.ln()/lambda;
    if let Some(max) = max_life {
        h.min(max)
    } else {
        h
    }
}



