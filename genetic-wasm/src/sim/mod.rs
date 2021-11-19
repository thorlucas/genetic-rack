use rand::Rng;
use wasm_bindgen::prelude::*;
use glam::*;
use crate::points::*;
use crate::gen::*;

//mod builder;
//pub use builder::SimBuilder;

pub const PHYSICS_MAX_FRAMERATE: f32 = 1.0 / 60.0;

#[wasm_bindgen]
pub struct Sim {
    points: PointPool,

    reciprocal_point_mass: f32,
    large_mass_gravity: f32,

    init_radius: GenRadius,
    init_momentum: GenMomentum,
    init_lifetime: GenLifetime,

    acc_dt: f32,
}

//#[wasm_bindgen]
impl Sim {
    //pub fn build() -> SimBuilder {
        //SimBuilder::new()
    //}

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
        self.acc_dt += dt;
        if !(self.acc_dt >= PHYSICS_MAX_FRAMERATE) {
            return;
        }
        let dt = self.acc_dt;
        self.acc_dt = 0.0;

        for p in self.points.iter_mut() {
            let dr = self.reciprocal_point_mass * *p.momentum * dt;
            //log(format!("dr: {:?}", dr).as_str());
            let dp = - self.large_mass_gravity / p.position.length().powf(3.0) * *p.position;
            //log(format!("dp: {:?}", dr).as_str());

            *p.position += dr;
            //log(format!("new pos: {:?}", *p.position).as_str());
            *p.momentum += dp;
            //log(format!("new mom: {:?}", *p.momentum).as_str());
            p.tick_lifetime(dt);
        }
    }
}
