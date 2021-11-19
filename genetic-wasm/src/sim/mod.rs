use rand::Rng;
use wasm_bindgen::prelude::*;
use glam::*;
use crate::utils::*;
use crate::points::*;

mod builder;
pub use builder::SimBuilder;

#[wasm_bindgen]
pub struct Sim {
    points: usize,
    max_points: usize,

    positions: Vec<Vec3>,
    momenta: Vec<Vec3>,
    life_time: Vec<f32>,
    alive: Vec<bool>,

    reciprocal_point_mass: f32,
    large_mass_gravity: f32,

    min_radius: f32,
    max_radius: f32,

    min_perp_momentum: f32,
    max_perp_momentum: f32,

    half_life: Option<f32>,
    max_life: Option<f32>,
}

#[wasm_bindgen]
impl Sim {
    pub fn build() -> SimBuilder {
        SimBuilder::new()
    }

    pub fn points_buffer_ptr(&self) -> *const f32 {
        self.positions.as_ptr() as *const f32
    }

    pub fn momentum_buffer_ptr(&self) -> *const f32 {
        self.momenta.as_ptr() as *const f32
    }

    pub fn spawn_points(&mut self, count: usize) {
        let count = count.min(self.max_points - self.points);
        self.points += count;

        let mut rng = rand::thread_rng();
        for _ in 0..count {
            // TODO: Get next dead
            let dir = rand_unit();
            let perp = dir.cross(rand_unit()).normalize();
            self.positions.push(dir * (rng.gen::<f32>() * (self.max_radius - self.min_radius) + self.min_radius));
            self.momenta.push(perp * (rng.gen::<f32>() * (self.max_perp_momentum - self.min_perp_momentum) + self.min_perp_momentum));
            self.alive.push(true);
            self.life_time.push({
                if let Some(half_life) = self.half_life {
                    rand_lifetime(half_life, self.max_life)
                } else {
                    std::f32::NAN
                }
            });
        }
    }

    pub fn tick(&mut self, dt: f32) {
        for i in 0..self.max_points {
            // TODO: Make more efficient using a linked list approach
            if !self.alive[i] {
                continue;
            }

            self.life_time[i] -= dt;
            if self.life_time[i] <= 0.0 {
                self.alive[i] = false;
                self.positions[i] = Vec3::ZERO;
                continue;
            }

            let dr = self.reciprocal_point_mass * self.momenta[i] * dt;
            let dp = - self.large_mass_gravity / self.positions[i].length().powf(3.0) * self.positions[i];

            self.positions[i] += dr;
            self.momenta[i] += dp;
        }
    }
}
