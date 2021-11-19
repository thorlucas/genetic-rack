mod utils;

use std::f32::consts::{PI, TAU};

use rand::Rng;
use wasm_bindgen::prelude::*;
use glam::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const RAND_SPEED: f32 = 10.0;

fn rand_unit() -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta = (1.0 - 2.0 * rng.gen::<f32>()).acos();
    let phi = TAU * rng.gen::<f32>();
    
    let rot = Mat3::from_rotation_ypr(theta, phi, 0.0);
    rot * Vec3::Y
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

        for _ in 0..n_points {
            points.push(rand_unit() * rng.gen::<f32>() * 40.0)
        }
        
        return Self {
            points,
        }
    }

    pub fn points_buffer_ptr(&self) -> *const f32 {
        self.points.as_ptr() as *const f32
    }

    pub fn random_walk(&mut self, dt: f32) {
        let mut rng = rand::thread_rng();

        for p in &mut self.points {
            p.x += (rng.gen::<f32>() - 0.5) * dt * RAND_SPEED;
            p.y += (rng.gen::<f32>() - 0.5) * dt * RAND_SPEED;
            p.z += (rng.gen::<f32>() - 0.5) * dt * RAND_SPEED;
        }
    }
}
