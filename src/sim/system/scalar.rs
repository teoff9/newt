use crate::sim::geom::vec3::Vec3;
use rayon::prelude::*;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct System {
    t: f64,
    n: usize,
    pos: Vec<Vec3>,
    vel: Vec<Vec3>,
    mass: Vec<f64>,
}

impl System {
    pub fn from(pos: Vec<Vec3>, vel: Vec<Vec3>, mass: Vec<f64>) -> Self {
        Self { t: 0.0, n: pos.len(), pos, vel, mass }
    }
    pub fn measure_kinetic(&self) -> f64 {
        self.mass.iter().zip(self.vel.iter()).fold(0.0, |k, (m, v)| k + 0.5 * *m * v.abs2())
    }

    pub fn measure_potential(&self, e2: f64, g: f64) -> f64 {
        -g * (0..self.n - 1)
            .into_par_iter()
            .fold(
                || 0.0,
                |mut v_tmp, i| {
                    (i + 1..self.n).for_each(|j| {
                        v_tmp += self.mass[i] * self.mass[j] / ((self.pos[i] - self.pos[j]).abs2() + e2).sqrt();
                    });
                    v_tmp
                },
            )
            .reduce(|| 0.0, |v_1, v_2| v_1 + v_2)
    }

    pub fn measure_e(&self, e2: f64, g: f64) -> f64 {
        self.measure_kinetic() + self.measure_potential(e2, g)
    }

    pub fn evolve(&mut self, g: f64, dt: f64, e2: f64, steps: i32) {
        let mut acc_2: Vec<Vec3>;
        let hald_dt = dt / 2.;

        //calculate initial acc
        let mut acc_1 = update_acc(&self.pos, &self.mass, &g, &e2, &self.n);

        //main loop
        for _ in 1..=steps {
            //Update positions
            update_pos(&mut self.pos, &self.vel, &acc_1, &dt);

            //Calculate new accellerations
            acc_2 = update_acc(&self.pos, &self.mass, &g, &e2, &self.n);

            //Update velocities
            update_vel(&mut self.vel, &acc_1, &acc_2, &hald_dt);

            //Swap acc_1 e acc_2, update t
            swap(&mut acc_1, &mut acc_2);
            acc_2.fill(Vec3::zero());
        }
        self.t = dt * steps as f64;
    }
}

#[inline(always)]
fn update_acc(pos: &[Vec3], mass: &[f64], g: &f64, e2: &f64, n: &usize) -> Vec<Vec3> {
    (0..*n)
        .into_par_iter()
        .fold(
            || vec![Vec3::zero(); *n],
            |mut acc_tmp, i| {
                let mut a: Vec3;
                for j in i + 1..*n {
                    let r = pos[i] - pos[j];
                    let rsq = r.abs2() + *e2;
                    let rinv = 1.0 / rsq.sqrt();
                    let rinv3 = rinv * rinv * rinv;
                    a = r * (*g * rinv3);

                    acc_tmp[i] += a * mass[j];
                    acc_tmp[j] -= a * mass[i];
                }
                acc_tmp
            },
        )
        .reduce(
            || vec![Vec3::zero(); *n],
            |mut acc1, acc2| {
                acc1.iter_mut().zip(acc2.iter()).for_each(|(a1, a2)| *a1 += *a2);
                acc1
            },
        )
}

#[inline(always)]
fn update_pos(pos: &mut [Vec3], vel: &[Vec3], acc: &[Vec3], dt: &f64) {
    let half_dt2 = 0.5 * *dt * *dt;
    for ((p, v), a) in pos.iter_mut().zip(vel).zip(acc) {
        *p += *v * *dt + *a * half_dt2;
    }
}

#[inline(always)]
fn update_vel(vel: &mut [Vec3], acc_1: &[Vec3], acc_2: &[Vec3], half_dt: &f64) {
    for ((v, a1), a2) in vel.iter_mut().zip(acc_1).zip(acc_2) {
        *v += (*a1 + *a2) * *half_dt
    }
}
