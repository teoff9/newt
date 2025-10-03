use crate::sim::geom::{Half, Vec3};
use num_traits::Float;
use rayon::prelude::*;
use std::mem::swap;
use std::ops::{AddAssign, MulAssign, SubAssign};

pub struct System<T: Float> {
    t: T,
    n: usize,
    pos: Vec<Vec3<T>>,
    vel: Vec<Vec3<T>>,
    mass: Vec<T>,
}

impl<T: Float + AddAssign + SubAssign + MulAssign + Send + Sync + Half> System<T> {
    pub fn from(pos: Vec<Vec3<T>>, vel: Vec<Vec3<T>>, mass: Vec<T>) -> Self {
        Self {
            t: T::zero(),
            n: pos.len(),
            pos,
            vel,
            mass,
        }
    }
    pub fn measure_kinetic(&self) -> T {
        self.mass.iter().zip(self.vel.iter()).fold(T::zero(), |k, (m, v)| k + T::half() * *m * v.abs2())
    }

    pub fn measure_potential(&self, e2: T, g: T) -> T {
        -g * (0..self.n - 1)
            .into_par_iter()
            .fold(
                || T::zero(),
                |mut v_tmp, i| {
                    (i + 1..self.n).for_each(|j| {
                        v_tmp += self.mass[i] * self.mass[j] / ((self.pos[i] - self.pos[j]).abs2() + e2).sqrt();
                    });
                    v_tmp
                },
            )
            .reduce(|| T::zero(), |v_1, v_2| v_1 + v_2)
    }

    pub fn measure_e(&self, e2: T, g: T) -> T {
        self.measure_kinetic() + self.measure_potential(e2, g)
    }

    pub fn evolve(&mut self, g: T, dt: T, e2: T, steps: i32) {
        let mut acc_2: Vec<Vec3<T>>;

        //calculate initial acc
        let mut acc_1 = update_acc(&self.pos, &self.mass, &g, &e2, &self.n);

        //main loop
        for _ in 1..=steps {
            //Update positions
            update_pos(&mut self.pos, &self.vel, &acc_1, &dt);

            //Calculate new accellerations
            acc_2 = update_acc(&self.pos, &self.mass, &g, &e2, &self.n);

            //Update velocities
            update_vel(&mut self.vel, &acc_1, &acc_2, &dt);

            //Swap acc_1 e acc_2, update t
            swap(&mut acc_1, &mut acc_2);
            acc_2.fill(Vec3::zero());
        }
        self.t = dt * T::from(steps).unwrap();
    }
}

#[inline]
fn update_acc<T: Float + AddAssign + SubAssign + MulAssign + Send + Sync + Half>(pos: &[Vec3<T>], mass: &[T], g: &T, e2: &T, n: &usize) -> Vec<Vec3<T>> {
    (0..*n)
        .into_par_iter()
        .fold(
            || vec![Vec3::zero(); *n],
            |mut acc_tmp, i| {
                let mut a: Vec3<T>;
                for j in i + 1..*n {
                    let r = pos[i] - pos[j];
                    a = r * *g / (r.abs2() + *e2).sqrt().powi(3);
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

#[inline]
fn update_pos<T: Float + AddAssign + SubAssign + MulAssign + Send + Sync + Half>(pos: &mut [Vec3<T>], vel: &[Vec3<T>], acc: &[Vec3<T>], dt: &T) {
    pos.iter_mut().enumerate().for_each(|(i, p)| *p += vel[i] * *dt + acc[i] * T::half() * *dt * *dt);
}

#[inline]
fn update_vel<T: Float + AddAssign + SubAssign + MulAssign + Send + Sync + Half>(vel: &mut [Vec3<T>], acc_1: &[Vec3<T>], acc_2: &[Vec3<T>], dt: &T) {
    vel.iter_mut().enumerate().for_each(|(i, v)| *v += (acc_2[i] + acc_1[i]) * *dt * T::half());
}
