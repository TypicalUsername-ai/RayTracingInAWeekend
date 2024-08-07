use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::velem::VElem;

pub trait Material<T: VElem> {
    fn scatter(&self, ray_in: Ray<T>, hit: HitRecord<T>, attenuation: Color<T>) -> Ray<T>;
}
