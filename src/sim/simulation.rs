use crate::sim::config::{read_config, write_config, Bodies, Config};
use anyhow::Result;
use std::mem::swap;

#[derive(Debug, PartialEq)]
pub struct Simulation {
    g: f64,
    dt: f64,
    steps: i64,
    softening: f64,
    pos: Vec<[f64; 3]>,
    vel: Vec<[f64; 3]>,
    mass: Vec<f64>,
}

impl Simulation {
    pub fn from_config(c: Config) -> Self {
        Self {
            g: c.g,
            dt: c.dt,
            steps: c.steps,
            softening: c.softening,
            pos: c.bodies.pos,
            vel: c.bodies.vel,
            mass: c.bodies.mass,
        }
    }

    pub fn from_config_file(path: &str) -> anyhow::Result<Self> {
        let conf = read_config(path)?;
        Ok(Self::from_config(conf))
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let c = Config {
            g: self.g,
            dt: self.dt,
            steps: self.steps,
            softening: self.softening,
            bodies: Bodies {
                pos: self.pos.clone(),
                vel: self.vel.clone(),
                mass: self.mass.clone(),
            },
        };
        write_config(path, &c)
    }

    pub fn from(
        g: f64,
        dt: f64,
        steps: i64,
        softening: f64,
        pos: Vec<[f64; 3]>,
        vel: Vec<[f64; 3]>,
        mass: Vec<f64>,
    ) -> Self {
        Self {
            g,
            dt,
            steps,
            softening,
            pos,
            vel,
            mass,
        }
    }

    pub fn set_g(&mut self, g: f64) {
        self.g = g;
    }

    pub fn set_dt(&mut self, dt: f64) {
        self.dt = dt;
    }

    pub fn set_steps(&mut self, steps: i64) {
        self.steps = steps;
    }

    pub fn set_softening(&mut self, softening: f64) {
        self.softening = softening;
    }

    pub fn total_kinetic(&self) -> f64 {
        todo!()
    }

    pub fn total_angular_mom(&self) -> f64 {
        todo!()
    }

    //Run simulation with velocity verlet for n steps
    pub fn run(&mut self) {
        let n = self.pos.len(); //n bodies
        let mut acc_1 = vec![[0.0; 3]; n]; //i acc
        let mut acc_2 = vec![[0.0; 3]; n]; //i+1 acc

        //calculate initial acc
        update_acc(&self.pos, &self.mass, &mut acc_1, &self.g);

        //main loop
        for i in 1..=self.steps {
            //Update positions
            update_pos(&mut self.pos, &self.vel, &acc_1, &self.dt);

            //Calculate new accellerations
            update_acc(&self.pos, &self.mass, &mut acc_2, &self.g);

            //Update velocities
            update_vel(&mut self.vel, &acc_1, &acc_2, &self.dt);

            //Swap acc_1 e acc_2
            swap(&mut acc_1, &mut acc_2);
        }
    }
}

fn update_acc(pos: &Vec<[f64; 3]>, mass: &Vec<f64>, acc: &mut Vec<[f64; 3]>, g: &f64) {}

fn update_pos(pos: &mut Vec<[f64; 3]>, vel: &Vec<[f64; 3]>, acc: &Vec<[f64; 3]>, dt: &f64) {}

fn update_vel(vel: &mut Vec<[f64; 3]>, acc_1: &Vec<[f64; 3]>, acc_2: &Vec<[f64; 3]>, dt: &f64) {}
