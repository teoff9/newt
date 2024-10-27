//27.10.24 by Matteo Fava
use std::ops::{Add, Sub, Mul};

//Vector 2d
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    vx: f64, 
    vy: f64,
    vz: f64,
}

impl Vector {
    pub fn vx(&self) -> f64 {
        self.vx
    }
    pub fn vy(&self) -> f64 {
        self.vy
    }
    pub fn vz(&self) -> f64 {
        self.vz
    }

    pub fn module(&self) -> f64 {
        ( self.vx*self.vx + self.vy*self.vy + self.vz*self.vz ).sqrt()
    }

    pub fn new(vx: f64, vy: f64, vz: f64) -> Self {
        Self {vx,vy, vz}
    }

    pub fn from(xyz: (f64, f64, f64)) -> Self {
        Self {vx: xyz.0, vy: xyz.1, vz: xyz.2}
    }
    pub fn set(&mut self, xyz: (f64, f64, f64)) {
        self.vx = xyz.0;
        self.vy = xyz.1;
        self.vz = xyz.2;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
            vz: self.vz + rhs.vz
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            vx: self.vx - rhs.vx,
            vy: self.vy - rhs.vy,
            vz: self.vz - rhs.vz
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            vx: self.vx * rhs,
            vy: self.vy * rhs,
            vz: self.vz * rhs
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            vx: self.vx * rhs as f64,
            vy: self.vy * rhs as f64,
            vz: self.vz * rhs as f64
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            vx: self.vx * rhs as f64,
            vy: self.vy * rhs as f64,
            vz: self.vz * rhs as f64
        }
    }
}