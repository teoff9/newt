use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn from(v: [f64; 3]) -> Self {
        Self(v)
    }
    pub fn zero() -> Self {
        Self([0.; 3])
    }
    pub fn to_array(&self) -> [f64; 3] {
        self.0
    }

    pub fn abs(&self) -> f64 {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt()
    }

    pub fn abs2(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self[i] *= rhs[i];
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3([rhs[0] * self, rhs[1] * self, rhs[2] * self])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self[i] /= rhs;
        }
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self[i] *= rhs;
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self[i] -= rhs[i];
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        for i in 0..3 {
            self[i] += rhs[i];
        }
    }
}
