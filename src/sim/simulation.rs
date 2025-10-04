use crate::sim::system::System;

#[derive(Debug, Clone)]
pub struct Sim {
    dt: f64,
    g: f64,
    e: f64,
    sys: System,
}

impl Sim {
    pub fn from(dt: f64, g: f64, e: f64, sys: System) -> Self {
        Self { dt, g, e, sys }
    }

    pub fn sys(&self) -> &System {
        &self.sys
    }

    pub fn run(&mut self, steps: i32) {
        self.sys.evolve(self.g, self.dt, self.e * self.e, steps);
    }
}
