use serde::Deserialize;
use crate::{gen::*, gravity::{GravitySim, GravitySimOpts}, physics::{tick, Hamiltonian}, points::PointPool};
use wasm_bindgen::prelude::*;

pub const PHYSICS_MAX_FRAMERATE: f32 = 1.0 / 60.0;

#[derive(Deserialize)]
#[serde(default)]
#[wasm_bindgen]
pub struct Opts {
    initial_points: usize,
    max_points: usize,
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
            gravity_opts: Default::default(),
            radius: FixedOrRange::Fixed(30.0),
            momentum: FixedOrRange::Fixed(80.0),
            lifetime: GenLifetime::Fixed(15.0),
        }
    }
}

#[wasm_bindgen]
pub struct Sim {
    gravity: GravitySim,
    points: PointPool,
    init_radius: GenRadius,
    init_momentum: GenMomentum,
    init_lifetime: GenLifetime,
    physics_dt: f32,
}

#[wasm_bindgen]
impl Sim {
    pub fn new(opts: Opts) -> Self {
        let gravity = GravitySim::new(opts.gravity_opts);
        let points = PointPool::with_capacity(opts.max_points);
        
        let mut sim = Self {
            gravity,
            points,
            init_radius: opts.radius,
            init_momentum: opts.momentum,
            init_lifetime: opts.lifetime,
            physics_dt: 0.0,
        };

        sim.spawn_points(opts.initial_points);
        
        sim
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
        self.physics_dt += dt;
        let physics_tick: bool = self.physics_dt >= PHYSICS_MAX_FRAMERATE;

        for mut p in self.points.iter_mut() {
            if physics_tick {
                tick(&self.gravity, &mut p.position, &mut p.momentum, self.physics_dt);
            }
            p.tick_lifetime(dt);
        }


        if physics_tick {
            self.physics_dt = 0.0; 
        }
    }
}
