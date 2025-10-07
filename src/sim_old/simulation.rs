use crate::sim_old::{
    config::{read_config, write_config, Bodies, Config},
    geom::Vec3,
};
use anyhow::Result;
use rand::{rng, Rng};
use rayon::prelude::*;
use std::mem::swap;

#[derive(Debug, PartialEq, Clone)]
pub struct Simulation {
    pub g: f64,
    pub dt: f64,
    pub steps: i64,
    pub softening: f64,
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

    pub fn system(&self) -> (Vec<Vec3>, Vec<Vec3>, Vec<f64>) {
        (self.pos.clone(), self.vel.clone(), self.mass.clone())
    }

    pub fn random_system(g: f64, dt: f64, steps: i64, n: usize, softening: f64, mass_r: (f64, f64), pos_r: [(f64, f64); 3], vel_r: Option<[(f64, f64); 3]>) -> Self {
        let mut rng = rng();
        let mut pos = vec![Vec3::zero(); n];
        let mut mass = vec![0.0; n];
        let mut vel = vec![Vec3::zero(); n];

        let (mi, mf) = mass_r;
        let [(xi, xf), (yi, yf), (zi, zf)] = pos_r;
        let vel_ranges = vel_r.unwrap_or([(0., 0.); 3]);
        let [(vxi, vxf), (vyi, vyf), (vzi, vzf)] = vel_ranges;

        for i in 0..n {
            // massa
            mass[i] = if mi != mf { rng.random_range(mi..mf) } else { mi };

            pos[i] = Vec3::from([rng.random_range(xi..xf), rng.random_range(yi..yf), rng.random_range(zi..zf)]);

            vel[i] = if vel_r.is_some() {
                Vec3::from([rng.random_range(vxi..vxf), rng.random_range(vyi..vyf), rng.random_range(vzi..vzf)])
            } else {
                Vec3::zero()
            };
        }

        Self {
            g,
            dt,
            steps,
            softening,
            mass,
            pos,
            vel,
        }
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

    pub fn from(g: f64, dt: f64, steps: i64, softening: f64, pos: Vec<Vec3>, vel: Vec<Vec3>, mass: Vec<f64>) -> Self {
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
        self.mass.iter().zip(self.vel.iter()).fold(0.0, |k, (m, v)| k + 0.5 * m * v.abs2())
    }

    pub fn total_potential(&self) -> f64 {
        let e = self.softening * self.softening;
        let n = self.pos.len();

        -self.g
            * (0..n - 1)
                .into_par_iter()
                .fold(
                    || 0.0,
                    |mut v_tmp, i| {
                        (i + 1..n).for_each(|j| {
                            v_tmp += self.mass[i] * self.mass[j] / ((self.pos[i] - self.pos[j]).abs2() + e).sqrt();
                        });
                        v_tmp
                    },
                )
                .reduce(|| 0.0, |v_1, v_2| v_1 + v_2)
    }

    pub fn total_e(&self) -> f64 {
        self.total_kinetic() + self.total_potential()
    }

    pub fn run(&mut self) {
        let n = self.pos.len(); //n bodies
        let e = self.softening * self.softening;
        let mut acc_1 = vec![Vec3::zero(); n]; //i acc
        let mut acc_2 = vec![Vec3::zero(); n]; //i+1 acc
        let mut a = Vec3::zero();

        //calculate initial acc
        update_acc(&self.pos, &self.mass, &mut acc_1, &mut a, &self.g, &e, n);

        //main loop
        for _ in 1..=self.steps {
            //Update positions
            update_pos(&mut self.pos, &self.vel, &acc_1, &self.dt);

            //Calculate new accellerations
            update_acc(&self.pos, &self.mass, &mut acc_2, &mut a, &self.g, &e, n);

            //Update velocities
            update_vel(&mut self.vel, &acc_1, &acc_2, &self.dt);

            //Swap acc_1 e acc_2
            swap(&mut acc_1, &mut acc_2);
            acc_2.fill(Vec3::zero());
        }
    }

    pub fn par_run(&mut self) {
        let n = self.pos.len(); //n bodies
        let e = self.softening * self.softening;
        let mut acc_2: Vec<Vec3>;

        //calculate initial acc
        let mut acc_1 = par_update_acc(&self.pos, &self.mass, &self.g, &e, n);

        //main loop
        for _ in 1..=self.steps {
            //Update positions
            update_pos(&mut self.pos, &self.vel, &acc_1, &self.dt);

            //Calculate new accellerations
            acc_2 = par_update_acc(&self.pos, &self.mass, &self.g, &e, n);

            //Update velocities
            update_vel(&mut self.vel, &acc_1, &acc_2, &self.dt);

            //Swap acc_1 e acc_2
            swap(&mut acc_1, &mut acc_2);
            acc_2.fill(Vec3::zero());
        }
    }
}

#[inline]
fn update_acc(pos: &[Vec3], mass: &[f64], acc: &mut [Vec3], a: &mut Vec3, g: &f64, e: &f64, n: usize) {
    for i in 0..n {
        for j in i + 1..n {
            let r = pos[i] - pos[j];
            *a = r * *g / (r.abs2() + e).sqrt().powi(3);
            acc[i] += mass[j] * *a;
            acc[j] -= mass[i] * *a;
        }
    }
}

#[inline]
fn par_update_acc(pos: &[Vec3], mass: &[f64], g: &f64, e: &f64, n: usize) -> Vec<Vec3> {
    (0..n)
        .into_par_iter()
        .fold(
            || vec![Vec3::zero(); n],
            |mut acc_tmp, i| {
                let mut a: Vec3;
                for j in i + 1..n {
                    let r = pos[i] - pos[j];
                    a = r * *g / (r.abs2() + e).sqrt().powi(3);
                    acc_tmp[i] += mass[j] * a;
                    acc_tmp[j] -= mass[i] * a;
                }
                acc_tmp
            },
        )
        .reduce(
            || vec![Vec3::zero(); n],
            |mut acc1, acc2| {
                acc1.iter_mut().zip(acc2.iter()).for_each(|(a1, a2)| *a1 += *a2);
                acc1
            },
        )
}

#[inline]
fn update_pos(pos: &mut [Vec3], vel: &[Vec3], acc: &[Vec3], dt: &f64) {
    pos.iter_mut().enumerate().for_each(|(i, p)| *p += vel[i] * *dt + 0.5 * acc[i] * *dt * *dt);
}

#[inline]
fn update_vel(vel: &mut [Vec3], acc_1: &[Vec3], acc_2: &[Vec3], dt: &f64) {
    vel.iter_mut().enumerate().for_each(|(i, v)| *v += (acc_2[i] + acc_1[i]) * *dt * 0.5);
}
