use glam::Vec3;
use crate::{memory::{BufferF32, Component}, utils::*};

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
        //log(format!("ticking lifetime: {} for index: {}", *self.lifetime, self.index).as_str());
        *self.lifetime -= dt;
        if *self.lifetime <= 0.0 {
            self.pool.kill_index(self.index);
        }
    }
}

enum PointLife {
    Alive { lifetime: f32 },
    Dead { next: Option<usize> },
}

pub struct PointPool {
    capacity: usize,
    positions: Vec<Vec3>,
    momenta: Vec<Vec3>,
    lives: Vec<PointLife>,
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
            next_dead: match capacity {
                0 => None,
                _ => Some(0),
            }
        }
    }

    pub fn spawn(&mut self, position: Vec3, momentum: Vec3, lifetime: f32) -> Result<()> {
        if let Some(index) = self.next_dead {
            if let PointLife::Dead { next } = self.lives[index] {
                console_log!("spawning: {}\n\told next dead: {:?}\n\tnew next dead: {:?}", index, self.next_dead, next);
                self.next_dead = next;
                self.positions[index] = position;
                self.momenta[index] = momentum;
                self.lives[index] = PointLife::Alive { lifetime };
                Ok(())
            } else {
                panic!("Point should be dead!");
            }
        } else {
            Err(PointPoolError::ReachedCapacity)
        }
    }

    fn kill_index(&mut self, index: usize) {
        if let PointLife::Alive { lifetime: _ } = self.lives[index] {
            console_log!("killing: {}\n\told next dead: {:?}\n\tnew next dead: {:?}\n\told pos: {:?}", index, self.next_dead, index, self.positions[index]);
            self.positions[index] = Vec3::ZERO;
            self.lives[index] = PointLife::Dead { next: self.next_dead };
            self.next_dead = Some(index);
        } else {
            panic!("Point should be alive!");
        }
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a> {
        IterMut {
            index: 0,
            pool: self,
        }
    }

    //pub fn point_pos_buffer(&self) -> Float32InterleavedBuffer {
        //Float32InterleavedBuffer {
            //buffer_ptr: self.positions.as_ptr() as *const f32,
            //stride: 3,
            //offset: 0,
            //items: self.capacity,
        //}
    //}
    pub fn buffers(&self) -> Vec<BufferF32> {
        vec![
            BufferF32::new(
                Component::point(&[
                    ("position", 3),
                ]),
                self.capacity,
                self.positions.as_slice().into(),
            ),
            BufferF32::new(
                Component::point(&[
                    ("momentum", 3),
                ]),
                self.capacity,
                self.momenta.as_slice().into(),
            )
        ]
    }
}

pub struct IterMut<'a> {
    pool: &'a mut PointPool,
    index: usize,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = PointMutRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.pool.capacity {
                return None;
            } else if let PointLife::Alive { ref mut lifetime } = self.pool.lives[self.index] {
                let index = self.index;
                self.index += 1;

                //log("---------------------------------------");
                //log(format!("iter: {}", index).as_str());
                //log(format!("lifetime in iter: {} for index {}", *lifetime, index).as_str());

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
            } else {
                self.index += 1;
            }
        }
    }
}
