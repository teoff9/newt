use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float>(pub [T; 3]);
impl<T: Float> Vec3<T> {
    pub fn from(v: [T; 3]) -> Self {
        Self(v)
    }
    pub fn zero() -> Self {
        Self([T::zero(); 3])
    }
    pub fn to_array(&self) -> [T; 3] {
        self.0
    }

    pub fn abs(&self) -> T {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt()
    }

    pub fn abs2(&self) -> T {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }
}

impl<T: Float> Index<usize> for Vec3<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Float> IndexMut<usize> for Vec3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Float + MulAssign> MulAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        for i in 0..3 {
            self[i] *= rhs[i];
        }
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl<T: Float + DivAssign> DivAssign<T> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self[i] /= rhs;
        }
    }
}

impl<T: Float + MulAssign> MulAssign<T> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self[i] *= rhs;
        }
    }
}

impl<T: Float> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl<T: Float + SubAssign> SubAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        for i in 0..3 {
            self[i] -= rhs[i];
        }
    }
}

impl<T: Float> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl<T: Float + AddAssign> AddAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3<T>) {
        for i in 0..3 {
            self[i] += rhs[i];
        }
    }
}

pub trait Half {
    fn half() -> Self;
}
impl Half for f64 {
    fn half() -> Self {
        0.5
    }
}
impl Half for f32 {
    fn half() -> Self {
        0.5
    }
}
