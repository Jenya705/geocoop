use crate::vec2::Vec2f;

#[derive(Clone, Copy, Default, Debug)]
pub struct Rectangle {
    min: Vec2f,
    max: Vec2f,
}

impl Rectangle {

    pub fn new(first_corner: Vec2f, second_corner: Vec2f) -> Rectangle {
        Rectangle {
            min: Vec2f::new(
                first_corner.x.min(second_corner.x),
                first_corner.y.min(second_corner.y),
            ),
            max: Vec2f::new(
                first_corner.x.max(second_corner.x),
                first_corner.y.max(second_corner.y),
            ),
        }
    }

    pub fn get_min(&self) -> Vec2f {
        self.min
    }

    pub fn get_max(&self) -> Vec2f {
        self.max
    }

    pub fn set_min(&mut self, min: Vec2f) {
        self.min = min
    }

    pub fn set_max(&mut self, max: Vec2f) {
        self.max = max
    }

    pub fn get_size(&mut self) -> Vec2f {
        Vec2f::new(self.max.x - self.min.x, self.max.y - self.min.y)
    } 

    pub fn get_area(&mut self) -> f64 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }

}