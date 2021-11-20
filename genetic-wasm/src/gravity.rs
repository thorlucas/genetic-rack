use wasm_bindgen::prelude::*;
use glam::*;
use serde::Deserialize;
use crate::physics::Hamiltonian;

#[derive(Deserialize)]
#[serde(default)]
pub struct GravitySimOpts {
    pub point_mass: f32,
    pub large_mass: f32,
    pub grav_const: f32,
}

impl Default for GravitySimOpts {
    fn default() -> Self {
        Self {
            point_mass: 10.0,
            large_mass: 5000.0,
            grav_const: 10.0,
        }
    }
}

#[wasm_bindgen]
pub struct GravitySim {
    reciprocal_point_mass: f32,
    large_mass_gravity: f32,
}

impl GravitySim {
    pub fn new(opts: GravitySimOpts) -> Self {
        Self {
            reciprocal_point_mass: 1.0 / opts.point_mass,
            large_mass_gravity: opts.large_mass * opts.grav_const,
        }
    }
}

impl Hamiltonian for GravitySim {
    fn dh_dp(&self, r: &Vec3, p: &Vec3) -> Vec3 {
        self.reciprocal_point_mass * (*p)
    }

    fn dh_dr(&self, r: &Vec3, p: &Vec3) -> Vec3 {
        (self.large_mass_gravity / r.length().powf(3.0)) * (*r)
    }
}
