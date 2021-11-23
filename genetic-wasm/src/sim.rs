use serde::Deserialize;
use crate::{gen::*, gravity::{GravitySim, GravitySimOpts}, memory::{BufferF32, Component, IBufferF32}, physics::{Kinetic, tick}, points::PointPool};
use wasm_bindgen::prelude::*;

pub const PHYSICS_MAX_FRAMERATE: f32 = 1.0 / 60.0;

#[derive(Deserialize)]
#[serde(default)]
#[wasm_bindgen]
pub struct Opts {
    initial_points: usize,
    max_points: usize,
    point_mass: f32,
    #[serde(flatten)]
    gravity_opts: GravitySimOpts,
    radius: GenRadius,
    momentum: GenMomentum,
    lifetime: GenLifetime,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            initial_points: 10,
            max_points: 100,
            point_mass: 10.0,
            gravity_opts: Default::default(),
            radius: GenRadius::Fixed(30.0),
            momentum: GenMomentum::Fixed(80.0),
            lifetime: GenLifetime::Fixed(15.0),
        }
    }
}

#[wasm_bindgen]
pub struct Sim {
    gravity: GravitySim,
    kinetic: Kinetic,
    points: PointPool,
    init_radius: GenRadius,
    init_momentum: GenMomentum,
    init_lifetime: GenLifetime,
}

#[wasm_bindgen]
impl Sim { 
    pub fn new(opts: Opts) -> Self {
        let gravity = GravitySim::new(opts.gravity_opts);
        let points = PointPool::with_capacity(opts.max_points);
        
        let mut sim = Self {
            gravity,
            kinetic: Kinetic::new(opts.point_mass),
            points,
            init_radius: opts.radius,
            init_momentum: opts.momentum,
            init_lifetime: opts.lifetime,
        };

        sim.spawn_points(opts.initial_points);
        
        sim
    }

    pub fn spawn_points(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count { 
            let position = gen_vec(&mut rng, &self.init_radius);
            let momentum = gen_perp_vec(&mut rng, &position, &self.init_momentum);
            let lifetime = self.init_lifetime.gen(&mut rng);
            self.points.spawn(
                position,
                momentum,
                lifetime,
            ).unwrap();
        }
    }

    pub fn tick(&mut self, dt: f32) {
        for mut p in self.points.iter_mut() {
            tick(&self.gravity, &mut p.position, &mut p.momentum, dt);
            tick(&self.kinetic, &mut p.position, &mut p.momentum, dt);
            p.tick_lifetime(dt);
        }
    }

    pub fn get_buffers(&self) -> Vec<IBufferF32> {
        let mut bufs = vec![];
        bufs.extend(self.gravity.buffers());
        bufs.extend(self.points.buffers());
        IBufferF32::from_bufs(bufs)
    }
}
