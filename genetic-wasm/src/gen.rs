use std::f32::consts::{LN_2, TAU};

use glam::{Mat3, Vec3};
use serde::Deserialize;
use rand::{Rng, RngCore};

pub trait Gen<T, R: RngCore> {
    fn gen(&self, rng: &mut R) -> T;
}

#[derive(Deserialize)]
pub struct Range {
    #[serde(default)]
    min: f32,
    max: f32,
}

impl<R: RngCore> Gen<f32, R> for Range {
    fn gen(&self, rng: &mut R) -> f32 {
        (self.max - self.min) * rng.gen::<f32>() + self.min
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FixedOrRange {
    Fixed(f32),
    Range(Range),
}

impl<R: RngCore> Gen<f32, R> for FixedOrRange {
    fn gen(&self, rng: &mut R) -> f32 {
        match self {
            Self::Fixed(r) => *r,
            Self::Range(r) => r.gen(rng),
        }
    }
}

pub type GenLength = FixedOrRange;
pub type GenRadius = FixedOrRange;
pub type GenMomentum = FixedOrRange;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GenLifetime {
    Fixed(f32),
    HalfLife {
        min: Option<f32>,
        max: Option<f32>,
        half_life: f32,
    }
}

impl<R: RngCore> Gen<f32, R> for GenLifetime {
    fn gen(&self, rng: &mut R) -> f32 {
        match *self {
            Self::Fixed(r) => r,
            Self::HalfLife { min, max, half_life } => {
                let lambda = LN_2 / half_life;
                let r: f32 = rng.gen();
                let mut h = -r.ln()/lambda;
                if let Some(min) = min {
                    h = h.min(min);
                }
                if let Some(max) = max {
                    h = h.max(max);
                }
                h
            }
        }
    }
}

pub fn gen_unit<R: RngCore>(rng: &mut R) -> Vec3 {
    let theta = (1.0 - 2.0 * rng.gen::<f32>()).acos();
    let phi = TAU * rng.gen::<f32>(); 
    let rot = Mat3::from_rotation_ypr(theta, phi, 0.0);
    rot * Vec3::Y
}

pub fn gen_vec<R: RngCore>(rng: &mut R, r: &GenLength) -> Vec3 {
    r.gen(rng) * gen_unit(rng)
}

pub fn gen_perp_unit<R: RngCore>(rng: &mut R, to: &Vec3) -> Vec3 {
    to.cross(gen_unit(rng)).normalize()
}

pub fn gen_perp_vec<R: RngCore>(rng: &mut R, to: &Vec3, r: &GenLength) -> Vec3 {
    gen_perp_unit(rng, to) * r.gen(rng)
}
