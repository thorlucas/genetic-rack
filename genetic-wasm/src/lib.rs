use sim::GravitySimOpts;
use wasm_bindgen::prelude::*;
use serde::Deserialize;

mod utils;
mod sim;
mod points;
mod gen;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use sim::GravitySim;
use utils::set_panic_hook;

#[derive(Deserialize)]
#[wasm_bindgen]
pub struct Opts {
    initial_points: usize,
    #[serde(flatten)]
    sim_opts: GravitySimOpts,
}

#[wasm_bindgen]
pub fn init(opts: &JsValue) -> GravitySim {
    set_panic_hook();
    let opts: Opts = opts.into_serde().unwrap();
    let mut sim = GravitySim::new(opts.sim_opts);
    sim.spawn_points(opts.initial_points);
    sim
}
