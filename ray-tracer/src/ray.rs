use crate::color::Color;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};
use crate::velem::VElem;

#[derive(Default, Clone, Copy)]
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
    pub fn new(origin: impl Into<Point3<T>>, direction: impl Into<Vec3<T>>) -> Self {
        Self {
            origin: origin.into(),
            direction: direction.into(),
        }
    }

    pub fn color(&self, world: &impl Hittable<T>, depth: u16) -> Color<T> {
        if depth <= 0 {
            return Color::default();
        }
        if let Some(hr) = world.hit(self, Into::<T>::into(0.0001)..=T::max_value()) {
            if let Some((scattered, attenuation)) = hr.material.scatter(self, &hr) {
                return attenuation * scattered.color(world, depth - 1);
            } else {
                return Color::from([T::zero(); 3]);
            }
        }
        let unit_direction = self.direction().unit_vector();
        let a: T = (unit_direction.y() + T::one()) * 0.5.into();
        let mut result = Color::from([T::one(); 3]);
        let scaler: T = T::one() - a;
        result *= scaler;
        result += Color::new(0.5.into(), 0.7.into(), 1.0.into()) * a;
        result
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
