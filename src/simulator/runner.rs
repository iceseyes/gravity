use crate::simulator::World;

pub struct Runner {
    world: World,
    time: f64,
}

impl Runner {
    pub fn new(world: World) -> Self {
        Self { world, time: 0.0 }
    }

    pub fn step(&mut self) {}
}
