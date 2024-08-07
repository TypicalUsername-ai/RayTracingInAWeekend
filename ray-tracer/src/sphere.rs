use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec3::Point3;
use crate::velem::VElem;

pub struct Sphere<T: VElem> {
    center: Point3<T>,
    radius: T,
    material: Box<dyn Material<T>>,
}

impl<T: VElem> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T, material: impl Material<T> + 'static) -> Self {
        Self {
            center,
            radius: T::max(0.0.into(), radius),
            material: Box::new(material),
        }
    }
}

impl<T: VElem> Hittable<T> for Sphere<T> {
    fn hit(
        &self,
        ray: &crate::ray::Ray<T>,
        ray_t: std::ops::RangeInclusive<T>,
    ) -> Option<HitRecord<T>> {
        let o_to_c = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&o_to_c);
        let c = o_to_c.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0.into() {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let root = (h - sqrtd) / a;
            if !ray_t.contains(&root) {
                let root = (h + sqrtd) / a;
                if !ray_t.contains(&root) {
                    return None;
                }
            }
            let outward_normal = (ray.at(root) - self.center) / self.radius;
            let mut hit_r = HitRecord {
                t: root,
                p: ray.at(root),
                normal: outward_normal,
                front_facing: false,
            };
            hit_r.set_face_normal(&ray, outward_normal);
            Some(hit_r)
        }
    }
}
