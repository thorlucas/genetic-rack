use sim::SimOptions;
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

pub use sim::Sim;
use utils::set_panic_hook;

#[derive(Deserialize)]
#[wasm_bindgen]
pub struct Opts {
    initial_points: usize,
    #[serde(flatten)]
    sim_opts: SimOptions,
}

#[wasm_bindgen]
pub fn init(opts: &JsValue) -> Sim {
    set_panic_hook();
    let opts: Opts = opts.into_serde().unwrap();
    let mut sim = Sim::new(opts.sim_opts);
    sim.spawn_points(opts.initial_points);
    sim
}
