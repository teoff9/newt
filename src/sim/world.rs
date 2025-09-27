//05.11.24 by Matteo Fava
use crate::{bodies::body::Body, geom::vec::Vector, physics::gravity::grav_field};

//World structs
#[derive(Debug)]
pub struct World {
    bodies: Vec<Body>,
    g: Vector
}

//IMPLEMENTATIONS
impl World {
    //empty
    pub fn empty() -> Self {
        Self {
            bodies: vec![], g: Vector::zero()
        }
    }
    
    //new
    pub fn new(bodies: Vec<Body>, g: Vector) -> Self {
        Self {
            bodies,g
        }
    }

    //replace method
    pub fn replace(&mut self, world: World) {
        self.bodies = world.bodies;
        self.g = world.g;
    }

    //add body
    pub fn add_body(&mut self, b: Body) {
        self.bodies.push(b);
    }

    //set new g
    pub fn set_g(&mut self, new_g: Vector) {
        self.g = new_g;
    }

    //Verlet integration
    pub fn step(&mut self, dt: &f64) {
        todo!()

    }

    //Euler step with velocities
    pub fn euler_step(&mut self, dt: &f64) {
        todo!()
    }

}

