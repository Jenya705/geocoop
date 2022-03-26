use std::cmp::Ordering;

use crate::{vec3::Vec3f, vec2::Vec2f};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VecOrdering {
    Less,
    Greater,
    Equal,
    Mixed,
}

pub trait VecCompare<T> {
    fn cmp(&self, second: &T) -> VecOrdering;
}

impl VecOrdering {

    pub fn is_less_or_eq(&self) -> bool {
        match self {
            VecOrdering::Less => true,
            VecOrdering::Greater => false,
            VecOrdering::Equal => true,
            VecOrdering::Mixed => false,
        }
    }

    pub fn is_greater_or_eq(&self) -> bool {
        match self { 
            VecOrdering::Less => false,
            VecOrdering::Greater => true,
            VecOrdering::Equal => true,
            VecOrdering::Mixed => false,
        }
    }

}

impl VecCompare<Vec3f> for Vec3f {
    fn cmp(&self, second: &Vec3f) -> VecOrdering {
        let x_ordering = self.x.total_cmp(&second.x);
        let y_ordering = self.y.total_cmp(&second.y);
        let z_ordering = self.z.total_cmp(&second.z);
        if x_ordering == y_ordering && x_ordering == z_ordering {
            return from_core_ordering(&x_ordering)
        }
        VecOrdering::Mixed
    }
}

impl VecCompare<Vec2f> for Vec2f {
    fn cmp(&self, second: &Vec2f) -> VecOrdering {
        let x_ordering = self.x.total_cmp(&second.x);
        let y_ordering = self.y.total_cmp(&second.y);
        if x_ordering == y_ordering {
            return from_core_ordering(&x_ordering)
        }
        VecOrdering::Mixed
    }
}

fn from_core_ordering(core_ordering: &Ordering) -> VecOrdering {
    return match core_ordering {
        Ordering::Less => VecOrdering::Less,
        Ordering::Equal => VecOrdering::Equal,
        Ordering::Greater => VecOrdering::Greater,
    }
} 

#[cfg(test)]
mod tests {
    use crate::{vec_cmp::{VecOrdering, VecCompare}, vec3::Vec3f, vec2::Vec2f};

    #[test]
    fn vec3f_tests() {
        assert_eq!(Vec3f::zero().cmp(&Vec3f::new(1.0, 2.0, 3.0)), VecOrdering::Less);
        assert_eq!(Vec3f::zero().cmp(&Vec3f::zero()), VecOrdering::Equal);
        assert_eq!(Vec3f::zero().cmp(&Vec3f::new(-1.0, -2.0, -3.0)), VecOrdering::Greater);
        assert_eq!(Vec3f::zero().cmp(&Vec3f::new(-1.0, 2.0, 0.0)), VecOrdering::Mixed)
    }

    #[test]
    fn vec2f_tests() {
        assert_eq!(Vec2f::zero().cmp(&Vec2f::new(1.0, 2.0)), VecOrdering::Less);
        assert_eq!(Vec2f::zero().cmp(&Vec2f::zero()), VecOrdering::Equal);
        assert_eq!(Vec2f::zero().cmp(&Vec2f::new(-1.0, -2.0)), VecOrdering::Greater);
        assert_eq!(Vec2f::zero().cmp(&Vec2f::new(-1.0, 2.0)), VecOrdering::Mixed);
    }

}