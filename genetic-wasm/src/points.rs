use glam::Vec3;
use crate::utils::*;

#[derive(Copy, Clone, Debug)]
pub enum PointPoolError {
    ReachedCapacity,
}

pub type Result<T> = std::result::Result<T, PointPoolError>;

pub struct PointMutRef<'a> {
    pub position: &'a mut Vec3,
    pub momentum: &'a mut Vec3,
    lifetime: &'a mut f32,
    pool: &'a mut PointPool,
    index: usize,
}

impl PointMutRef<'_> {
    pub fn tick_lifetime(self, dt: f32) {
        log(format!("ticking lifetime: {} for index: {}", *self.lifetime, self.index).as_str());
        *self.lifetime -= dt;
        if *self.lifetime <= 0.0 {
            self.pool.kill_index(self.index);
        }
    }
}

enum PointLife {
    Alive { lifetime: f32, prev: Option<usize>, next: Option<usize> },
    Dead { next: Option<usize> },
}

pub struct PointPool {
    capacity: usize,
    positions: Vec<Vec3>,
    momenta: Vec<Vec3>,
    lives: Vec<PointLife>,
    first_alive: Option<usize>,
    next_dead: Option<usize>,
}

impl<'a> PointPool {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut lives = Vec::with_capacity(capacity);
        for i in 0..capacity-1 {
            lives.push(PointLife::Dead { next: Some(i + 1) });
        }
        lives.push(PointLife::Dead { next: None });

        Self {
            capacity,
            positions: vec![Vec3::ZERO; capacity],
            momenta: vec![Vec3::ZERO; capacity],
            lives,
            first_alive: None,
            next_dead: match capacity {
                0 => None,
                _ => Some(0),
            }
        }
    }

    pub fn spawn(&mut self, position: Vec3, momentum: Vec3, lifetime: f32) -> Result<()> {
        if let Some(index) = self.next_dead {
            if let PointLife::Dead { next } = self.lives[index] {
                self.next_dead = next;
                self.positions[index] = position;
                self.momenta[index] = momentum;
                self.lives[index] = PointLife::Alive { lifetime, next: self.first_alive, prev: None };
                self.lives[self.first_alive]
                self.first_alive = Some(index);
                log(format!("--> inserted {}\n\tnew next: {:?}\n\tnew first: {:?}", index, self.next_dead, self.first_alive).as_str());
                Ok(())
            } else {
                panic!("Point should be dead!");
            }
        } else {
            Err(PointPoolError::ReachedCapacity)
        }
    }

    fn kill_index(&mut self, index: usize) {
        if let PointLife::Alive { lifetime: _, next, prev } = self.lives[index] {
            log(format!("killing {}:\n\tnext: {:?}\n\tprev: {:?}", index, next, prev).as_str());
            self.positions[index] = Vec3::ZERO;

            if let Some(next_index) = prev {
                if let PointLife::Alive { lifetime: _, next: ref mut prevs_next, prev: _ } = self.lives[next_index] {
                    *prevs_next = next;
                }
            } else {
                panic!("Point should be alive!");
            }

            if let Some(prev_index) = next {
                if let PointLife::Alive { lifetime: _, next: _, prev: ref mut nexts_prev } = self.lives[prev_index] {
                    *nexts_prev = prev;
                }
            } else {
                panic!("Point should be alive!");
            }

            self.lives[index] = PointLife::Dead { next: self.next_dead };
            self.next_dead = Some(index);
            log(format!("--> killed {}\n\tnew next: {:?}\n\tnew first: {:?}", index, self.next_dead, self.first_alive).as_str());
        } else {
            panic!("Point should be alive!");
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a> {
        IterMut {
            index: self.first_alive,
            pool: self,
        }
    }

    pub fn positions_as_ptr(&self) -> *const Vec3 {
        self.positions.as_ptr()
    }

    pub fn momenta_as_ptr(&self) -> *const Vec3 {
        self.momenta.as_ptr()
    }
}

pub struct IterMut<'a> {
    pool: &'a mut PointPool,
    index: Option<usize>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointMutRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index {
            if let PointLife::Alive { lifetime: ref mut lifetime, next, prev } = self.pool.lives[index] {
                log("---------------------------------------");
                log(format!("iter: {}", index).as_str());
                log(format!("lifetime in iter: {} for index {}", *lifetime, index).as_str());
                self.index = next;
                let p_ptr: *mut Vec3 = &mut self.pool.positions[index];
                let m_ptr: *mut Vec3 = &mut self.pool.momenta[index];
                let l_ptr: *mut f32 = lifetime;
                let pool_ptr: *mut PointPool = self.pool;
                unsafe {
                    return Some(PointMutRef {
                        position: &mut *p_ptr,
                        momentum: &mut *m_ptr,
                        lifetime: &mut *l_ptr,
                        pool: &mut *pool_ptr,
                        index,
                    });
                }
            }
        }
        None
    }
}
