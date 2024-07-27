use crate::color::Color;
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

    fn hit_sphere(&self, center: Point3<T>, radius: T) -> T {
        // origin to center vector
        let o_to_c = center - self.origin();
        // solving the discriminant
        let a = self.direction().length_squared();
        let h = self.direction().dot(&o_to_c);
        let c = o_to_c.length_squared() - radius * radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0.into() {
            (-1.0).into()
        } else {
            (h - discriminant.sqrt()) / a
        }
    }

    pub fn color(&self) -> Color<T> {
        let sphere_hit = self.hit_sphere(
            Point3::new(0.0.into(), 0.0.into(), (-1.0).into()),
            0.5.into(),
        );
        if sphere_hit > Into::<T>::into(0.0) {
            let norm = (self.at(sphere_hit) - Vec3::new(0.0.into(), 0.0.into(), (-1.0).into()))
                .unit_vector();
            return Color::new(
                norm.x() + 1.0.into(),
                norm.y() + 1.0.into(),
                norm.z() + 1.0.into(),
            ) * Into::<T>::into(0.5);
        }
        let unit_direction = self.direction().unit_vector();
        let a: T = (unit_direction.y() + 1.0.into()) * 0.5.into();
        let mut result = Color::new(1.0.into(), 1.0.into(), 1.0.into());
        let scaler: T = (<T as From<f32>>::from(1.0) - a).into();
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
