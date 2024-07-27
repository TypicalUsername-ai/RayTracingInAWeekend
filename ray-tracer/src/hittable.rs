use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::velem::VElem;

pub struct HitRecord<T: VElem> {
    pub p: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub front_facing: bool,
}

impl<T: VElem> HitRecord<T> {
    pub fn set_face_normal(&mut self, ray: Ray<T>, outward_normal: Vec3<T>) {
        self.front_facing = ray.direction().dot(&outward_normal) < Into::<T>::into(0.0);
        self.normal = if self.front_facing {
            outward_normal
        } else {
            Vec3::default() - outward_normal // vec3 is 0s
        };
    }
}

pub trait Hittable<T: VElem> {
    fn hit(&self, ray: Ray<T>, ray_t_min: T, ray_t_max: T) -> Option<HitRecord<T>> {
        None
    }
}
