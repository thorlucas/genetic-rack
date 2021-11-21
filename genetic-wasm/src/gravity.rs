use wasm_bindgen::prelude::*;
use glam::*;
use serde::Deserialize;
use crate::{memory::{BufferF32, Component, Point}, physics::Hamiltonian};

#[derive(Deserialize)]
#[serde(default)]
pub struct GravitySimOpts {
    pub grav_const: f32,
    pub init_sources: Vec<GravitySourceOpts>,
}

impl Default for GravitySimOpts {
    fn default() -> Self {
        Self {
            grav_const: 10.0,
            init_sources: vec![],
        }
    }
}

#[wasm_bindgen]
pub struct GravitySim {
    grav_const: f32,
    sources: Vec<GravitySource>,
}

impl GravitySim {
    pub fn new(opts: GravitySimOpts) -> Self {
        let grav_const: f32 = opts.grav_const;
        Self {
            grav_const,
            sources: {
                opts.init_sources.into_iter()
                    .map(|src| GravitySource::new(src, grav_const))
                    .collect()
            }
        }
    }

    pub fn add_source(&mut self, opts: GravitySourceOpts) {
        self.sources.push(GravitySource::new(opts, self.grav_const));
    }

    pub fn buffers(&self) -> Vec<BufferF32> {
        vec![
            BufferF32::new(
                Component::source(&[
                    ("position", 3),
                    ("mass", 1),
                ]),
                self.sources.len(),
                (&self.sources[..]).into()
            )
        ]
    }
}

impl Hamiltonian for GravitySim {
    // Optimization:
    // Since we know that gravity contributions never depend on momentum, we can skip this
    // NOTE: Change if this ever changes
    fn dh_dr(&self, r: &Vec3, p: &Vec3) -> Vec3 {
        (&self.sources[..]).dh_dr(r, p)
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[wasm_bindgen]
pub struct GravitySourceOpts {
    position: Vec3,
    mass: f32,
}

#[repr(C)]
struct GravitySource {
    position: Vec3,
    mass_gravity: f32,
}

impl GravitySource {
    pub fn new(opts: GravitySourceOpts, grav_const: f32) -> Self {
        Self {
            position: opts.position,
            mass_gravity: opts.mass * grav_const,
        }
    }
}

impl Hamiltonian for GravitySource {
    fn dh_dr(&self, r: &Vec3, _: &Vec3) -> Vec3 {
        let r = *r - self.position;
        (self.mass_gravity / r.length().powf(3.0)) * r
    }
}

