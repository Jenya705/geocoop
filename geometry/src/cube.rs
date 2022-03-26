use crate::vec3::Vec3f;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Cube {
    min: Vec3f,
    max: Vec3f,
}

impl Cube {

    pub fn new(first_corner: Vec3f, second_corner: Vec3f) -> Cube {
        Cube {
            min: Vec3f::new(
                first_corner.x.min(second_corner.x),
                first_corner.y.min(second_corner.y),
                first_corner.z.min(second_corner.z),
            ),
            max: Vec3f::new(
                first_corner.x.max(second_corner.x),
                first_corner.y.max(second_corner.y),
                first_corner.z.max(second_corner.z),
            ),
        }
    }

    pub fn get_min(&self) -> Vec3f {
        self.min
    }

    pub fn get_max(&self) -> Vec3f {
        self.max
    }

    pub fn set_min(&mut self, min: Vec3f) {
        self.min = min
    }

    pub fn set_max(&mut self, max: Vec3f) {
        self.max = max
    }

    pub fn get_size(&self) -> Vec3f {
        Vec3f::new(self.max.x - self.min.x, self.max.y - self.min.y, self.max.z - self.min.z)
    } 

    pub fn get_volume(&self) -> f64 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y) * (self.max.z - self.min.z)
    }

}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3f;

    use super::Cube;

    #[test]
    fn init_tests() {
        let cube = Cube::new(Vec3f::zero(), Vec3f::zero() + 2.0);
        assert_eq!(cube.get_min(), Vec3f::zero());
        assert_eq!(cube.get_max(), Vec3f::new(2.0, 2.0, 2.0));
        let cube = Cube::new(Vec3f::zero(), Vec3f::new(-2.0, 2.0, 1.0));
        assert_eq!(cube.get_min(), Vec3f::new(-2.0, 0.0, 0.0));
        assert_eq!(cube.get_max(), Vec3f::new(0.0, 2.0, 1.0));
    }

    #[test]
    fn size_tests() {
        let cube = Cube::new(Vec3f::zero(), Vec3f::zero() + 2.0);
        assert_eq!(cube.get_size(), Vec3f::new(2.0, 2.0, 2.0));
        let cube = Cube::new(Vec3f::zero() + 1.0, Vec3f::new(2.0, 3.0, 4.0));
        assert_eq!(cube.get_size(), Vec3f::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn volume_tests() {
        let cube = Cube::new(Vec3f::zero(), Vec3f::new(1.0, 2.0, 3.0));
        assert_eq!(cube.get_volume(), 6.0);
    }

}