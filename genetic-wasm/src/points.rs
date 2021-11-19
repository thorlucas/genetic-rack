use std::{borrow::BorrowMut, cell::{RefCell, RefMut}};
use glam::Vec3;

#[derive(Debug)]
pub struct PointHandle<'a>(usize, std::marker::PhantomData<&'a mut PointPool>);

pub struct PointMutRef<'a> {
    pub position: &'a mut Vec3,
    pub momentum: &'a mut Vec3,
    pub lifetime: &'a mut f32,
}

enum PointLife {
    Alive(f32, Option<usize>),
    Dead(Option<usize>),
}

pub struct PointPool {
    capacity: usize,
    positions: Vec<Vec3>,
    momenta: Vec<Vec3>,
    lives: Vec<PointLife>,
    first: Option<usize>,
    next: Option<usize>,
}

impl<'a> PointPool {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut lives = Vec::with_capacity(capacity);
        for i in 0..capacity-1 {
            lives.push(PointLife::Dead(Some(i + 1)));
        }
        lives.push(PointLife::Dead(None));

        Self {
            capacity,
            positions: vec![Vec3::ZERO; capacity],
            momenta: vec![Vec3::ZERO; capacity],
            lives,
            first: None,
            next: match capacity {
                0 => None,
                _ => Some(0),
            }
        }
    }

    pub fn spawn(&mut self, position: Vec3, momentum: Vec3, lifetime: f32) -> Result<PointHandle<'a>, ()> {
        if let Some(index) = self.next {
            if let PointLife::Dead(next) = self.lives[index] {
                self.next = next;
                self.positions[index] = position;
                self.momenta[index] = momentum;
                self.lives[index] = PointLife::Alive(lifetime, self.first);
                self.first = Some(index);
                Ok(PointHandle(index, std::marker::PhantomData))
            } else {
                panic!("Point should be dead!");
            }
        } else {
            Err(())
        }
    }

    fn kill_index(&mut self, index: usize) {
        if let PointLife::Alive(_, first) = self.lives[index] {
            self.first = first;
            self.positions[index] = Vec3::ZERO;
            self.next = Some(index);
        } else {
            panic!("Point should be alive!");
        }
    }

    pub fn kill_handle(&mut self, handle: PointHandle) {
        self.kill_index(handle.0);
    }

    fn position_mut(&mut self, index: usize) -> &mut Vec3 {
        &mut self.positions[index]
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a> {
        IterMut {
            index: self.first,
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
            if let PointLife::Alive(mut lifetime, next) = &mut self.pool.lives[index] {
                self.index = *next;
                let p_ptr: *mut Vec3 = &mut self.pool.positions[index];
                let m_ptr: *mut Vec3 = &mut self.pool.momenta[index];
                let l_ptr: *mut f32 = &mut lifetime;
                unsafe {
                    return Some(PointMutRef {
                        position: &mut *p_ptr,
                        momentum: &mut *m_ptr,
                        lifetime: &mut *l_ptr,
                    });
                }
                // TODO: Remove dead particles
            }
        }
        None
    }
}
