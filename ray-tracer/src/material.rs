use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::velem::VElem;

pub trait Material<T: VElem> {
    fn scatter(&self, ray_in: Ray<T>) -> Option<HitRecord<T>>;
}
