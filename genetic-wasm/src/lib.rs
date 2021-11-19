mod utils;

use std::f32::consts::{LN_2, PI, TAU};

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

fn rand_lifetime(half_life: f32, max_life: Option<f32>) -> f32 {
    let lambda = LN_2 / half_life;
    let r: f32 = rand::thread_rng().gen();
    let h = -r.ln()/lambda;
    if let Some(max) = max_life {
        h.min(max)
    } else {
        h
    }
}

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

#[wasm_bindgen]
pub struct SimBuilder {
    max_points: usize,
    initial_points: usize,

    max_radius: f32,
    min_radius: f32,

    max_perp_momentum: f32,
    min_perp_momentum: f32,

    point_mass: f32,
    large_mass: f32,
    gravity: f32,

    point_halflife: Option<f32>,
    point_maxlife: Option<f32>,
}

impl Default for SimBuilder {
    fn default() -> Self {
        Self {
            initial_points: 100,
            max_points: 1000,
            max_radius: 100.0,
            min_radius: 0.0,
            max_perp_momentum: 10.0,
            min_perp_momentum: 0.0,
            point_mass: 1.0,
            large_mass: 1000.0,
            gravity: 10.0,
            point_halflife: None,
            point_maxlife: None,
        }
    }
}

#[wasm_bindgen]
impl SimBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_points(mut self, n: usize) -> Self {
        self.max_points = n;
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

    pub fn point_halflife(mut self, h: f32) -> Self {
        self.point_halflife = Some(h);
        self
    }

    pub fn point_maxlife(mut self, l: f32) -> Self {
        self.point_maxlife = Some(l);
        self
    }

    pub fn create(self) -> Sim {
        let mut sim = Sim {
            points: 0,
            max_points: self.max_points,
            positions: Vec::with_capacity(self.max_points),
            momenta: Vec::with_capacity(self.max_points),
            alive: Vec::with_capacity(self.max_points),
            life_time: Vec::with_capacity(self.max_points),
            min_perp_momentum: self.min_perp_momentum,
            max_perp_momentum: self.max_perp_momentum,
            min_radius: self.min_radius,
            max_radius: self.max_radius,
            reciprocal_point_mass: 1.0 / self.point_mass,
            large_mass_gravity: self.large_mass * self.gravity,
            half_life: self.point_halflife,
            max_life: self.point_maxlife,
        };
        sim.spawn_points(self.initial_points);
        sim
    }
}
