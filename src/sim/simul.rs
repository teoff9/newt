//31.10.24 by Matteo Fava
use crate::{bodies::body::Body, geom::vec::Vector};

//SIMULATION: this is the core of Newt
#[derive(Debug)]
pub struct Simulation {
    pub bodies: Vec<Body>,
    grav_field: Vector,
    dt: f64,
}

//IMPLEMENTATIONS
impl Simulation {
    //New instance of simulation
    pub fn new(dt: f64, g: Vector) -> Self {
        Self {
            bodies: vec![],
            grav_field: g,
            dt,
        }
    }

    //add a body
    pub fn add_body(&mut self, b: Body) {
        self.bodies.push(b);
    }

    //set the integration parameter
    pub fn set_dt(&mut self, new_dt: f64) {
        self.dt = new_dt;
    }

    //start the physics simulation
    pub fn start(&mut self) {
        todo!()
    }

    //Uses Euler integration for first step if there's a body with v!=0
    fn first_step(&mut self) {
        todo!()
    }
    //normal step with Verlet integration
    fn step(&mut self) {
        todo!()
    }
}
