//27.10.24 by Matteo Fava
use crate::bodies::body::Body;
use crate::geom::vec::Vector;

#[derive(Debug)]
pub struct World {
    pub bodies: Vec<Body>,
    gravitational_field: Vector,
    electrical_field: Vector,
    dt: f64,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            gravitational_field: Vector::zero(),
            electrical_field: Vector::zero(),
            dt: 0.0
        }
    }
    pub fn from(bodies: Vec<Body>, g: Vector, e: Vector, dt: f64) -> Self {
        Self {
            bodies, gravitational_field: g, electrical_field: e, dt
        }
    }
    pub fn g(&self) -> &Vector {
        &self.gravitational_field
    }
    pub fn e(&self) -> &Vector {
        &self.electrical_field
    }
    pub fn dt(&self) -> &f64 {
        &self.dt
    }
    pub fn add_body(&mut self, b: Body) {
        self.bodies.push(b);
    }
    pub fn set_gravitational_field(&mut self, g: Vector) {
        self.gravitational_field = g;
    }
    pub fn set_electrical_field(&mut self, e: Vector) {
        self.electrical_field = e;
    }
    pub fn set_dt(&mut self, new_dt: f64) {
        self.dt = new_dt;
    }
}