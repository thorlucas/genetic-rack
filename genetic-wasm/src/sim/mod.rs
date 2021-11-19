use rand::Rng;
use wasm_bindgen::prelude::*;
use glam::*;
use crate::utils::*;
use crate::points::*;

mod builder;
pub use builder::SimBuilder;

#[wasm_bindgen]
pub struct Sim {
    points: PointPool,

    reciprocal_point_mass: f32,
    large_mass_gravity: f32,

    min_radius: f32,
    max_radius: f32,

    min_perp_momentum: f32,
    max_perp_momentum: f32,

    half_life: Option<f32>,
    max_life: Option<f32>,
}

//#[wasm_bindgen]
impl Sim {
    pub fn build() -> SimBuilder {
        SimBuilder::new()
    }

    pub fn positions_buffer_ptr(&self) -> *const f32 {
        self.points.positions_as_ptr() as *const f32
    }

    pub fn momenta_buffer_ptr(&self) -> *const f32 {
        self.points.momenta_as_ptr() as *const f32
    }

    pub fn spawn_points(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let dir = rand_unit();
            let perp = dir.cross(rand_unit()).normalize();
            
            let pos = dir * (rng.gen::<f32>() * (self.max_radius - self.min_radius) + self.min_radius);
            let mom = perp * (rng.gen::<f32>() * (self.max_perp_momentum - self.min_perp_momentum) + self.min_perp_momentum);
            let life = match self.half_life {
                None => match self.max_life {
                    None => std::f32::INFINITY,
                    Some(m) => m,
                },
                Some(h) => rand_lifetime(h, self.max_life),
            };

            self.points.spawn(pos, mom, life).unwrap();
        }
    }

    pub fn tick(&mut self, dt: f32) {
        for p in self.points.iter_mut() {
            let dr = self.reciprocal_point_mass * *p.momentum * dt;
            let dp = - self.large_mass_gravity / p.position.length().powf(3.0) * *p.position;

            *p.position += dr;
            *p.momentum += dp;
            *p.lifetime -= dt;
        }
    }
}
