use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {

    pub fn new(x: f64, y: f64) -> Vec2f {
        Vec2f {x, y}
    }

    pub fn zero() -> Vec2f {
        Vec2f::new(0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn abs(&self) -> Vec2f {
        Vec2f::new(self.x.abs(), self.y.abs())
    }

}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2f::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2f::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2f::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2f::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign for Vec2f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign for Vec2f {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Add<f64> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: f64) -> Self::Output {
        Vec2f::new(self.x + rhs, self.y + rhs)
    }
}

impl Sub<f64> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec2f::new(self.x - rhs, self.y - rhs)
    }
}

impl Mul<f64> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2f::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2f::new(self.x / rhs, self.y / rhs)
    }
}

impl AddAssign<f64> for Vec2f {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign<f64> for Vec2f {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<f64> for Vec2f {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f64> for Vec2f {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Add<Vec2f> for f64 {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Self::Output {
        Vec2f::new(self + rhs.x, self + rhs.y)
    }
}

impl Sub<Vec2f> for f64 {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Self::Output {
        Vec2f::new(self - rhs.x, self - rhs.y)
    }
}

impl Mul<Vec2f> for f64 {
    type Output = Vec2f;

    fn mul(self, rhs: Vec2f) -> Self::Output {
        Vec2f::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<Vec2f> for f64 {
    type Output = Vec2f;

    fn div(self, rhs: Vec2f) -> Self::Output {
        Vec2f::new(self / rhs.x, self / rhs.y)
    }
}

impl Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
        Vec2f::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2::Vec2f;


    #[test]
    fn init_tests() {
        let vec1 = Vec2f::new(1.0, 2.0);
        assert_eq!(vec1.x, 1.0);
        assert_eq!(vec1.y, 2.0);
        let zero = Vec2f::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
    }

    #[test]
    fn eq_tests() {
        assert_eq!(Vec2f::new(1.0, 2.0), Vec2f::new(1.0, 2.0));
        assert_ne!(Vec2f::new(1.0, 2.0), Vec2f::zero());
    }

    #[test]
    fn vec_calc_tests() {
        let vec1 = Vec2f::new(1.0, 2.0);
        let vec2 = Vec2f::new(3.0, 2.0);
        assert_eq!(vec1 + vec2, Vec2f::new(4.0, 4.0));
        assert_eq!(vec1 - vec2, Vec2f::new(-2.0, 0.0));
        assert_eq!(vec1 * vec2, Vec2f::new(3.0, 4.0));
        assert_eq!(vec1 / vec2, Vec2f::new(1.0 / 3.0, 1.0));
    }

    #[test]
    fn vec_f64_calc_tests() {
        let vec1 = Vec2f::new(1.0, 2.0);
        assert_eq!(vec1 + 2.0, Vec2f::new(3.0, 4.0));
        assert_eq!(vec1 - 2.0, Vec2f::new(-1.0, 0.0));
        assert_eq!(vec1 * 2.0, Vec2f::new(2.0, 4.0));
        assert_eq!(vec1 / 2.0, Vec2f::new(0.5, 1.0));
    }

    #[test]
    fn vec_spec_calc_tests() {
        let vec1 = Vec2f::new(-1.0, 2.0);
        assert_eq!(-vec1, Vec2f::new(1.0, -2.0));
        assert_eq!(vec1.abs(), Vec2f::new(1.0, 2.0));
        assert_eq!((-vec1).abs(), vec1.abs());
    }

}