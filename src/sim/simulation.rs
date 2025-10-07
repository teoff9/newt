use crate::sim::system::scalar::System;

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

    pub fn kinetic(&self) -> f64 {
        self.sys().measure_kinetic()
    }

    pub fn potential(&self) -> f64 {
        self.sys().measure_potential(self.e * self.e, self.g)
    }

    pub fn energy(&self) -> f64 {
        self.sys().measure_e(self.e * self.e, self.g)
    }

    pub fn run(&mut self, steps: i32) {
        self.sys.evolve(self.g, self.dt, self.e * self.e, steps);
    }
}
