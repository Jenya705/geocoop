use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {x, y, z}
    }

    pub fn zero() -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn abs(&self) -> Vec3f {
        Vec3f::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3f::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3f::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Add<f32> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3f::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f32> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec3f::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Add<Vec3f> for f32 {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Self::Output {
        Vec3f::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Sub<Vec3f> for f32 {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Self::Output {
        Vec3f::new(self - rhs.x, self - rhs.y, self - rhs.z)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        Vec3f::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<Vec3f> for f32 {
    type Output = Vec3f;

    fn div(self, rhs: Vec3f) -> Self::Output {
        Vec3f::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Self::Output {
        Vec3f::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec3f;

    #[test]
    fn init_tests() {
        let vec1 = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 2.0);
        assert_eq!(vec1.z, 3.0);
        let zero = Vec3f::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
        assert_eq!(zero.z, 0.0);
    }

    #[test]
    fn eq_tests() {
        assert_eq!(Vec3f::new(1.0, 2.0, 3.0), Vec3f::new(1.0, 2.0, 3.0));
        assert_ne!(Vec3f::new(1.0, 2.0, 3.0), Vec3f::zero());
    }

    #[test]
    fn vec_calc_tests() {
        let vec1 = Vec3f::new(1.0, 2.0, 3.0);
        let vec2 = Vec3f::new(3.0, 2.0, 1.0);
        assert_eq!(vec1 + vec2, Vec3f::new(4.0, 4.0, 4.0));
        assert_eq!(vec1 - vec2, Vec3f::new(-2.0, 0.0, 2.0));
        assert_eq!(vec1 * vec2, Vec3f::new(3.0, 4.0, 3.0));
        assert_eq!(vec1 / vec2, Vec3f::new(1.0 / 3.0, 1.0, 3.0));
    }

    #[test]
    fn vec_f32_calc_tests() {
        let vec1 = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(vec1 + 2.0, Vec3f::new(3.0, 4.0, 5.0));
        assert_eq!(vec1 - 2.0, Vec3f::new(-1.0, 0.0, 1.0));
        assert_eq!(vec1 * 2.0, Vec3f::new(2.0, 4.0, 6.0));
        assert_eq!(vec1 / 2.0, Vec3f::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn vec_spec_calc_tests() {
        let vec1 = Vec3f::new(-1.0, 2.0, -4.0);
        assert_eq!(-vec1, Vec3f::new(1.0, -2.0, 4.0));
        assert_eq!(vec1.abs(), Vec3f::new(1.0, 2.0, 4.0));
        assert_eq!((-vec1).abs(), vec1.abs());
    }

}