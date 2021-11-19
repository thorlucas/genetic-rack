use glam::Vec3;

pub struct PointHandle<'a>(usize, std::marker::PhantomData<&'a PointPool>);

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

    pub fn get_mut(&mut self, handle: PointHandle<'a>) -> PointMutRef {
        if let PointLife::Alive(lifetime, _) = &mut self.lives[handle.0] {
            PointMutRef {
                position: &mut self.positions[handle.0],
                momentum: &mut self.momenta[handle.0],
                lifetime,
            }
        } else {
            panic!("Invalid handle!");
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

    pub fn kill(&mut self, handle: PointHandle<'a>) {
        if let PointLife::Alive(_, first) = self.lives[handle.0] {
            self.first = first;
            self.positions[handle.0] = Vec3::ZERO;
            self.next = Some(handle.0);
        } else {
            panic!("Point should be alive!");
        }
    }

    pub fn iter(&'a self) -> Iter<'a> {
        Iter {
            pool: self,
            index: self.first,
        }
    }
}

pub struct Iter<'a> {
    pool: &'a PointPool,
    index: Option<usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PointHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index {
            if let PointLife::Alive(_, next) = self.pool.lives[index] {
                self.index = next;
                return Some(PointHandle(index, std::marker::PhantomData));
            }
        }
        None
    }
}
