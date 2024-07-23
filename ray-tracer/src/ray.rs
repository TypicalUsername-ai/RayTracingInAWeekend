use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[inline]
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, time: f32) -> Point3 {
        self.origin + (self.direction * time)
    }
    pub fn new(origin: impl Into<[f32; 3]>, direction: impl Into<[f32; 3]>) -> Self {
        Self {
            origin: Point3::from(origin),
            direction: Vec3::from(direction),
        }
    }
}

#[cfg(test)]
mod impl_tests {

    use super::*;

    #[test]
    fn accessor_tests() {
        let r = Ray::new([1.0, 1.0, 1.0], [2.0, 1.0, 1.0]);

        assert_eq!(r.origin().x(), 1.0);
        assert_eq!(r.origin().y(), 1.0);
        assert_eq!(r.origin().z(), 1.0);

        assert_eq!(r.direction().x(), 2.0);
        assert_eq!(r.direction().y(), 1.0);
        assert_eq!(r.direction().z(), 1.0);
    }

    #[test]
    fn at_tests() {
        let r = Ray::new([1.0, 1.0, 1.0], [2.0, 1.0, 0.5]).at(2.0);
        assert_eq!(r.x(), 5.0);
        assert_eq!(r.y(), 3.0);
        assert_eq!(r.z(), 2.0);
    }
}
