use crate::{cube::Cube, vec_cmp::VecCompare, rectangle::Rectangle};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CollideResult {
    Intersects,
    In,
    Contains,
    None
}

pub trait Collide<T> {
    fn collide(&self, other: &T) -> CollideResult;
}

fn collide<T: VecCompare<T>>(min1: &T, max1: &T, min2: &T, max2: &T) -> CollideResult {
    let min_min_ordering = min1.cmp(min2);
    let min_max_ordering = min1.cmp(max2);
    let max_min_ordering = max1.cmp(min2);
    let max_max_ordering = max1.cmp(max2);
    let min_min_less = min_min_ordering.is_less_or_eq();
    let min_min_greater = min_min_ordering.is_greater_or_eq();
    let min_max_less = min_max_ordering.is_less_or_eq();
    // let min_max_greater = min_max_ordering.is_greater_or_eq(); <--- can not be
    // let max_min_less = max_min_ordering.is_less_or_eq(); <--- can not be
    let max_min_greater = max_min_ordering.is_greater_or_eq();
    let max_max_less = max_max_ordering.is_less_or_eq();
    let max_max_greater = max_max_ordering.is_greater_or_eq();
    if min_min_less && max_max_greater {
        return CollideResult::Contains
    }
    if min_min_greater && max_max_less {
        return CollideResult::In
    }
    if min_min_greater && max_max_greater && min_max_less {
        return CollideResult::Intersects
    }
    if min_min_less && max_max_less && max_min_greater {
        return CollideResult::Intersects
    }
    if min_max_less && max_min_greater {
        return CollideResult::Intersects
    }
    CollideResult::None
}

impl Collide<Cube> for Cube {
    fn collide(&self, other: &Cube) -> CollideResult {
        collide(&self.get_min(), &self.get_max(), &other.get_min(), &other.get_max())
    }
}

impl Collide<Rectangle> for Rectangle {
    fn collide(&self, other: &Rectangle) -> CollideResult {
        collide(&self.get_min(), &self.get_max(), &other.get_min(), &other.get_max())
    }
}

#[cfg(test)]
mod tests {
    use crate::{cube::Cube, vec3::Vec3f, collide::{CollideResult, Collide}, rectangle::Rectangle, vec2::Vec2f};

    #[test]
    fn cube_tests() {
        let cube1 = Cube::new(Vec3f::zero() + 1.0, Vec3f::zero() + 2.0);
        let cube2 = Cube::new(Vec3f::zero(), Vec3f::zero() + 3.0);
        assert_eq!(cube1.collide(&cube2), CollideResult::In);
        assert_eq!(cube2.collide(&cube1), CollideResult::Contains);
        let cube2 = Cube::new(Vec3f::zero(), Vec3f::zero() + 1.5);
        assert_eq!(cube1.collide(&cube2), CollideResult::Intersects);
        assert_eq!(cube2.collide(&cube1), CollideResult::Intersects);
        let cube2 = Cube::new(Vec3f::zero() + 3.0, Vec3f::zero() + 4.0);
        assert_eq!(cube1.collide(&cube2), CollideResult::None);
        assert_eq!(cube2.collide(&cube1), CollideResult::None);
        let cube2 = Cube::new(Vec3f::new(1.5, 1.5, 0.5), Vec3f::new(3.0, 2.0, 3.5));
        assert_eq!(cube1.collide(&cube2), CollideResult::Intersects);
        assert_eq!(cube2.collide(&cube1), CollideResult::Intersects);
    }

    #[test]
    fn rectangle_tests() {
        let rect1 = Rectangle::new(Vec2f::zero() + 1.0, Vec2f::zero() + 2.0);
        let rect2 = Rectangle::new(Vec2f::zero(), Vec2f::zero() + 3.0);
        assert_eq!(rect1.collide(&rect2), CollideResult::In);
        assert_eq!(rect2.collide(&rect1), CollideResult::Contains);
        let rect2 = Rectangle::new(Vec2f::zero(), Vec2f::zero() + 1.5);
        assert_eq!(rect1.collide(&rect2), CollideResult::Intersects);
        assert_eq!(rect2.collide(&rect1), CollideResult::Intersects);
        let rect2 = Rectangle::new(Vec2f::zero() + 3.0, Vec2f::zero() + 4.0);
        assert_eq!(rect1.collide(&rect2), CollideResult::None);
        assert_eq!(rect2.collide(&rect1), CollideResult::None);
        let rect2 = Rectangle::new(Vec2f::new(1.5, 1.5), Vec2f::new(3.0, 2.0));
        assert_eq!(rect1.collide(&rect2), CollideResult::Intersects);
        assert_eq!(rect2.collide(&rect1), CollideResult::Intersects);
    }

}