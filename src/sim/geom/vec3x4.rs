use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use wide::f64x4;

#[derive(Debug, Clone, Copy)]
#[repr(align(32))]
pub struct Vec3x4 {
    x: f64x4,
    y: f64x4,
    z: f64x4,
}

impl Vec3x4 {
    #[inline(always)]
    pub fn from(v: [f64x4; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }

    #[inline(always)]
    pub fn zero() -> Self {
        let zero = f64x4::splat(0.);
        Self { x: zero, y: zero, z: zero }
    }

    #[inline(always)]
    pub fn abs2(&self) -> f64x4 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn abs(&self) -> f64x4 {
        self.abs2().sqrt()
    }
}

impl Mul<f64x4> for Vec3x4 {
    type Output = Vec3x4;
    #[inline(always)]
    fn mul(self, rhs: f64x4) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64x4> for Vec3x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64x4) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64x4> for Vec3x4 {
    type Output = Vec3x4;
    #[inline(always)]
    fn div(self, rhs: f64x4) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64x4> for Vec3x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64x4) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul<Vec3x4> for f64x4 {
    type Output = Vec3x4;
    #[inline(always)]
    fn mul(self, rhs: Vec3x4) -> Vec3x4 {
        Vec3x4 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Add<Vec3x4> for Vec3x4 {
    type Output = Vec3x4;
    #[inline(always)]
    fn add(self, rhs: Vec3x4) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vec3x4> for Vec3x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3x4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3x4> for Vec3x4 {
    type Output = Vec3x4;
    #[inline(always)]
    fn sub(self, rhs: Vec3x4) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vec3x4> for Vec3x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec3x4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3x4> for Vec3x4 {
    type Output = Vec3x4;
    #[inline(always)]
    fn mul(self, rhs: Vec3x4) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<Vec3x4> for Vec3x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vec3x4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
