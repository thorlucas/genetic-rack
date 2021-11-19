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
    n: usize,

    rs: Vec<Vec3>,
    ps: Vec<Vec3>,

    mr: f32,
    gm: f32,
}

#[wasm_bindgen]
impl Sim {
    pub fn build() -> SimBuilder {
        SimBuilder::new()
    }

    pub fn points_buffer_ptr(&self) -> *const f32 {
        self.rs.as_ptr() as *const f32
    }

    pub fn momentum_buffer_ptr(&self) -> *const f32 {
        self.ps.as_ptr() as *const f32
    }

    pub fn random_walk(&mut self, dt: f32) {
        let mut rng = rand::thread_rng();

        for p in &mut self.rs {
            p.x += (rng.gen::<f32>() - 0.5) * dt * RAND_SPEED;
            p.y += (rng.gen::<f32>() - 0.5) * dt * RAND_SPEED;
            p.z += (rng.gen::<f32>() - 0.5) * dt * RAND_SPEED;
        }
    }

    pub fn orbit(&mut self, dt: f32) {
        for i in 0..self.n {
            let dr = self.mr * self.ps[i] * dt;
            let dp = - self.gm / self.rs[i].length().powf(3.0) * self.rs[i];

            self.rs[i] += dr;
            self.ps[i] += dp;
        }
    }
}

#[wasm_bindgen]
pub struct SimBuilder {
    n_points: usize,

    max_radius: f32,
    min_radius: f32,

    max_perp_momentum: f32,
    min_perp_momentum: f32,

    point_mass: f32,
    large_mass: f32,
    gravity: f32,
}

impl Default for SimBuilder {
    fn default() -> Self {
        Self {
            n_points: 1000,
            max_radius: 100.0,
            min_radius: 0.0,
            max_perp_momentum: 10.0,
            min_perp_momentum: 0.0,
            point_mass: 1.0,
            large_mass: 1000.0,
            gravity: 10.0,
        }
    }
}

#[wasm_bindgen]
impl SimBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn n_points(mut self, n: usize) -> Self {
        self.n_points = n;
        self
    }

    pub fn max_radius(mut self, r: f32) -> Self {
        self.max_radius = r;
        self
    }

    pub fn min_radius(mut self, r: f32) -> Self {
        self.min_radius = r;
        self
    }

    pub fn max_perp_momentum(mut self, p: f32) -> Self {
        self.max_perp_momentum = p;
        self
    }

    pub fn min_perp_momentum(mut self, p: f32) -> Self {
        self.min_perp_momentum = p;
        self
    }

    pub fn point_mass(mut self, m: f32) -> Self {
        self.point_mass = m;
        self
    }

    pub fn large_mass(mut self, m: f32) -> Self {
        self.large_mass = m;
        self
    }

    pub fn gravity(mut self, g: f32) -> Self {
        self.gravity = g;
        self
    }

    pub fn create(self) -> Sim {
        let mut rng = rand::thread_rng();

        let mut rs: Vec<Vec3> = Vec::with_capacity(self.n_points);
        let mut ps: Vec<Vec3> = Vec::with_capacity(self.n_points);

        for _ in 0..self.n_points {
            let dir = rand_unit();
            let perp = dir.cross(rand_unit()).normalize();
            rs.push(dir * (rng.gen::<f32>() * (self.max_radius - self.min_radius) + self.min_radius));
            ps.push(perp * (rng.gen::<f32>() * (self.max_perp_momentum - self.min_perp_momentum) + self.min_perp_momentum));
        }

        return Sim {
            n: self.n_points,
            rs,
            ps,
            mr: 1.0 / self.point_mass,
            gm: self.large_mass * self.gravity,
        };
    }
}
