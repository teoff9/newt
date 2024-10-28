//27.10.24 by Matteo Fava
use std::ops::{Add, Sub, Mul, Div};
use super::vector::Vector;

//Vector3D
#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    vx: f64, 
    vy: f64,
    vz: f64,
}

impl Vector for Vector3D {
    fn vx(&self) -> f64 {
        self.vx
    }
    fn vy(&self) -> f64 {
        self.vy
    }
    fn vz(&self) -> f64 {
        self.vz
    }
}

impl Vector3D {
    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self {vx,vy, vz}
    }
    pub fn from(v: &Vector3D) -> Self {
        Vector3D {
            vx: v.vx,
            vy: v.vy,
            vz: v.vz
        }
    }
    pub fn to_unit(&self) -> Self {
        Vector3D::from(self) / self.module()
    }
    pub fn module(&self) -> f64 {
        ( self.vx*self.vx + self.vy*self.vy + self.vz*self.vz ).sqrt()
    }
    pub fn set(&mut self, xyz: (f64, f64, f64)) {
        self.vx = xyz.0;
        self.vy = xyz.1;
        self.vz = xyz.2;
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3D {
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
            vz: self.vz + rhs.vz
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D {
            vx: self.vx - rhs.vx,
            vy: self.vy - rhs.vy,
            vz: self.vz - rhs.vz
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D {
            vx: self.vx * rhs,
            vy: self.vy * rhs,
            vz: self.vz * rhs
        }
    }
}

impl Mul<i32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector3D {
            vx: self.vx * rhs as f64,
            vy: self.vy * rhs as f64,
            vz: self.vz * rhs as f64
        }
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D {
            vx: self.vx * rhs as f64,
            vy: self.vy * rhs as f64,
            vz: self.vz * rhs as f64
        }
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3D {
            vx: self.vx / rhs,
            vy: self.vy / rhs,
            vz: self.vz / rhs
        }
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f32) -> Self::Output {
        Vector3D {
            vx: self.vx / rhs as f64,
            vy: self.vy / rhs as f64,
            vz: self.vz / rhs as f64
        }
    }
}

impl Div<i32> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: i32) -> Self::Output {
        Vector3D {
            vx: self.vx / rhs as f64,
            vy: self.vy / rhs as f64,
            vz: self.vz / rhs as f64
        }
    }
}