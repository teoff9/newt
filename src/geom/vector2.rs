//27.10.24 by Matteo Fava
use std::ops::{Add, Sub, Mul, Div};
use super::{vector::Vector, vector3::Vector3D};

//Vector2D 
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    vx: f64, 
    vy: f64
}

impl Vector for Vector2D {
    fn vx(&self) -> f64 {
        self.vx
    }
    fn vy(&self) -> f64 {
        self.vy
    }
}

impl Vector2D {
    pub fn new(vx: f64, vy: f64) -> Self {
        Self {vx,vy}
    }
    pub fn from(v: &Vector2D) -> Self {
        Vector2D {
            vx: v.vx,
            vy: v.vy,
        }
    }
    pub fn to_unit(&self) -> Self {
        Vector2D::from(self) / self.module()
    }
    pub fn to_vector3(&self) -> Vector3D {
        Vector3D::new(self.vx, self.vy, self.vz())
    }
    pub fn module(&self) -> f64 {
        ( self.vx*self.vx + self.vy*self.vy ).sqrt()
    }
    pub fn set(&mut self, xy: (f64, f64)) {
        self.vx = xy.0;
        self.vy = xy.1;
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
        }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            vx: self.vx - rhs.vx,
            vy: self.vy - rhs.vy,
        }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D {
            vx: self.vx * rhs,
            vy: self.vy * rhs,
        }
    }
}

impl Mul<i32> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector2D {
            vx: self.vx * rhs as f64,
            vy: self.vy * rhs as f64,
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D {
            vx: self.vx * rhs as f64,
            vy: self.vy * rhs as f64,
        }
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector2D {
            vx: self.vx / rhs,
            vy: self.vy / rhs,
        }
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2D {
            vx: self.vx / rhs as f64,
            vy: self.vy / rhs as f64,
        }
    }
}

impl Div<i32> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: i32) -> Self::Output {
        Vector2D {
            vx: self.vx / rhs as f64,
            vy: self.vy / rhs as f64,
        }
    }
}