use crate::gravity::GravitySim;

use glam::Vec3;

pub trait Hamiltonian {
    fn dh_dp(&self, r: &Vec3, p: &Vec3) -> Vec3;
    fn dh_dr(&self, r: &Vec3, p: &Vec3) -> Vec3;
}

impl<'a, T, I> Hamiltonian for T
    where T: Copy + IntoIterator<Item = &'a I>,
          I: 'a + Hamiltonian {
    fn dh_dp(&self, r: &Vec3, p: &Vec3) -> Vec3 {
        let mut acc = Vec3::ZERO;
        for h in self.into_iter() {
            acc += h.dh_dp(r, p);
        }
        acc
    }

    fn dh_dr(&self, r: &Vec3, p: &Vec3) -> Vec3 {
        let mut acc = Vec3::ZERO;
        for h in self.into_iter() {
            acc += h.dh_dr(r, p);
        }
        acc
    }
}

pub fn tick<H: Hamiltonian>(h: &H, r: &mut Vec3, p: &mut Vec3, dt: f32) {
    *r += h.dh_dp(r, p) * dt;
    *p -= h.dh_dr(r, p) * dt;
}
