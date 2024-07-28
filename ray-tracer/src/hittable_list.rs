use crate::hittable::Hittable;
use crate::velem::VElem;
use std::rc::Rc;

pub type HittableList<T> = Vec<Rc<dyn Hittable<T>>>;

impl<T: VElem> Hittable<T> for HittableList<T> {
    fn hit(
        &self,
        ray: &crate::ray::Ray<T>,
        ray_t: std::ops::RangeInclusive<T>,
    ) -> Option<crate::hittable::HitRecord<T>> {
        let mut closest_hit = None;
        let mut closest_hit_time = *ray_t.end();
        for object in self {
            if let Some(hit) = object.hit(&ray, *ray_t.start()..=closest_hit_time) {
                closest_hit_time = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
