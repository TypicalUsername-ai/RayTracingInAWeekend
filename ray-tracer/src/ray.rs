use crate::vec3::{Point3, Vec3};
use crate::velem::VElem;

#[derive(Default)]
pub struct Ray<T: VElem> {
    origin: Point3<T>,
    direction: Vec3<T>,
}

impl<T: VElem> Ray<T> {
    #[inline]
    pub fn origin(&self) -> Point3<T> {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vec3<T> {
        self.direction
    }

    pub fn at(&self, time: T) -> Point3<T> {
        self.origin + (self.direction * time)
    }
    pub fn new(origin: impl Into<[T; 3]>, direction: impl Into<[T; 3]>) -> Self {
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
