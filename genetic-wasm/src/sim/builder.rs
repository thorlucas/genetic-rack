use wasm_bindgen::prelude::*;
use crate::{points::PointPool, sim::Sim};

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

    pub fn initial_points(mut self, n: usize) -> Self {
        self.initial_points = n;
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
            points: PointPool::with_capacity(self.max_points),
            min_perp_momentum: self.min_perp_momentum,
            max_perp_momentum: self.max_perp_momentum,
            min_radius: self.min_radius,
            max_radius: self.max_radius,
            reciprocal_point_mass: 1.0 / self.point_mass,
            large_mass_gravity: self.large_mass * self.gravity,
            half_life: self.point_halflife,
            max_life: self.point_maxlife,
            acc_dt: 0.0,
        };
        sim.spawn_points(self.initial_points.min(self.max_points));
        sim
    }
}
