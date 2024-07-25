use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::velem::VElem;

pub struct HitRecord<T: VElem> {
    pub p: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
}

pub trait Hittable<T: VElem> {
    fn hit(&self, ray: Ray<T>, ray_t_min: T, ray_t_max: T) -> Option<HitRecord<T>> {
        None
    }
}
