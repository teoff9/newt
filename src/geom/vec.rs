//27.10.24 by Matteo Fava
use std::ops::{Add, Sub, Mul, Div, AddAssign};
use bevy::math::Vec3;


//Vector
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64, 
    y: f64,
    z: f64,
}

//METHODS
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x,y,z}
    }
    pub fn from(v: &Vector) -> Self {
        Vector {
            x: v.x,
            y: v.y,
            z: v.z
        }
    }
    pub fn zero() -> Self {
        Vector {
            x:0.0,y:0.0,z:0.0
        }
    }
    fn x(&self) -> &f64 {
        &self.x
    }
    fn y(&self) -> &f64 {
        &self.y
    }
    fn z(&self) -> &f64 {
        &self.z
    }
    pub fn to_unit(&self) -> Self {
        Vector::from(self) / self.module()
    }
    pub fn module(&self) -> f64 {
        ( self.x*self.x + self.y*self.y + self.z*self.z ).sqrt()
    }
    pub fn set(&mut self, xyz: (f64, f64, f64)) {
        self.x = xyz.0;
        self.y = xyz.1;
        self.z = xyz.2;
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}



//IMPLEMENTATIONS
impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64
        }
    }
}

impl Div<i32> for Vector {
    type Output = Vector;
    fn div(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64
        }
    }
}