use rand::Rng;
use wasm_bindgen::prelude::*;
use glam::*;
use serde::Deserialize;
use crate::points::*;
use crate::gen::*;

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
            large_mass: 500.0,
            grav_const: 1.0,
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

impl GravitySim { 
    pub fn tick(&mut self, dt: f32, p: &mut PointMutRef) {
        let dr = self.reciprocal_point_mass * *p.momentum * dt;
        //log(format!("dr: {:?}", dr).as_str());
        let dp = - self.large_mass_gravity / p.position.length().powf(3.0) * *p.position;
        //log(format!("dp: {:?}", dr).as_str());

        *p.position += dr;
        //log(format!("new pos: {:?}", *p.position).as_str());
        *p.momentum += dp;
        //log(format!("new mom: {:?}", *p.momentum).as_str());
    }
}
