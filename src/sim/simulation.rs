use crate::sim::{
    config::{read_config, write_config, Bodies, Config},
    geom::Vec3,
};
use anyhow::Result;
use std::mem::swap;

#[derive(Debug, PartialEq)]
pub struct Simulation {
    g: f64,
    dt: f64,
    steps: i64,
    softening: f64,
    pos: Vec<Vec3>,
    vel: Vec<Vec3>,
    mass: Vec<f64>,
}

impl Simulation {
    pub fn from_config(c: Config) -> Self {
        Self {
            g: c.g,
            dt: c.dt,
            steps: c.steps,
            softening: c.softening,
            pos: c.bodies.pos.iter().map(|v| Vec3::from(*v)).collect(),
            vel: c.bodies.vel.iter().map(|v| Vec3::from(*v)).collect(),
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
                pos: self.pos.iter().map(|v| v.to_array()).collect(),
                vel: self.vel.iter().map(|v| v.to_array()).collect(),
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
        pos: Vec<Vec3>,
        vel: Vec<Vec3>,
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
        let mut k = 0.0;
        for i in 0..self.pos.len() {
            k += 0.5 * self.mass[i] * self.vel[i].abs2();
        }
        k
    }

    //Run simulation with velocity verlet for n steps
    pub fn run(&mut self) {
        let n = self.pos.len(); //n bodies
        let mut acc_1 = vec![Vec3::zero(); n]; //i acc
        let mut acc_2 = vec![Vec3::zero(); n]; //i+1 acc

        //calculate initial acc
        update_acc(
            &self.pos,
            &self.mass,
            &mut acc_1,
            &self.g,
            &self.softening,
            n,
        );

        //main loop
        for _ in 1..=self.steps {
            //Update positions
            update_pos(&mut self.pos, &self.vel, &acc_1, &self.dt, n);

            //Calculate new accellerations
            update_acc(
                &self.pos,
                &self.mass,
                &mut acc_2,
                &self.g,
                &self.softening,
                n,
            );

            //Update velocities
            update_vel(&mut self.vel, &acc_1, &acc_2, &self.dt, n);

            //Swap acc_1 e acc_2
            swap(&mut acc_1, &mut acc_2);
        }
    }
}

#[inline]
fn update_acc(pos: &[Vec3], mass: &[f64], acc: &mut [Vec3], g: &f64, softening: &f64, n: usize) {
    let mut a: Vec3;
    let e = softening * softening;
    for i in 0..n {
        a = Vec3::zero();
        for j in 0..n {
            if i != j {
                let r = pos[i] - pos[j];
                a += r * *g * mass[j] / (r.abs2() + e).sqrt().powi(3);
            }
        }
        acc[i] = a;
    }
}

#[inline]
fn update_pos(pos: &mut [Vec3], vel: &[Vec3], acc: &[Vec3], dt: &f64, n: usize) {
    for i in 0..n {
        pos[i] += vel[i] * *dt + 0.5 * acc[i] * *dt * *dt;
    }
}

#[inline]
fn update_vel(vel: &mut [Vec3], acc_1: &[Vec3], acc_2: &[Vec3], dt: &f64, n: usize) {
    for i in 0..n {
        vel[i] += (acc_2[i] + acc_1[i]) * *dt * 0.5;
    }
}
