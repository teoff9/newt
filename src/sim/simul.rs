//31.10.24 by Matteo Fava
use crate::{bodies::body::Body, geom::vec::Vector, sim::world::World};
use std::{sync::{Arc, Mutex}, thread};
use std::time::Duration;

//2d or 3d
#[derive(Debug)]
enum Dimension {
    TwoD,
    ThreeD
}

//SIMULATION: this is the core of Newt
#[derive(Debug)]
pub struct Simulation {
    pub world: Arc<Mutex<World>>,
    dt: f64,
}

//IMPLEMENTATIONS
impl Simulation {
    //New with
    pub fn new_with(dt: f64, bodies: Vec<Body>, g: Vector) -> Self {
        Self {
            dt,
            world: Arc::new(Mutex::new(World::new(bodies, g)))
        }
    }

    //New instance of simulation
    pub fn new(dt: f64) -> Self {
        Self {
            world: Arc::new(Mutex::new(World::empty())),
            dt
        }
    }

    //add world method
    pub fn add_world(&mut self, new_wrld: World) {
        self.world.lock().expect("Can't lock world").replace(new_wrld);
    }

    //set the integration parameter
    pub fn set_dt(&mut self, new_dt: f64) {
        self.dt = new_dt;
    }

    //start the physics simulation
    pub fn run(&mut self, dim: Dimension) {
        todo!()
    }
}
