use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::velem::VElem;

pub struct Metal<T: VElem> {
    albedo: Color<T>,
    fuzz: T,
}

impl<T: VElem> Metal<T> {
    pub fn new(albedo: Color<T>, fuzz: T) -> Self {
        let fuzz = fuzz.max(T::one());
        Self { albedo, fuzz }
    }
}

impl<T: VElem> Material<T> for Metal<T> {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray<T>,
        hit: &crate::hittable::HitRecord<T>,
    ) -> Option<(crate::ray::Ray<T>, crate::color::Color<T>)> {
        let reflected = if self.fuzz == T::zero() {
            ray_in.direction().reflect(hit.normal)
        } else {
            ray_in.direction().reflect(hit.normal) + Vec3::random_unit_vec() * self.fuzz
        };
        let scattered = Ray::new(hit.p, reflected);
        if scattered.direction().dot(&hit.normal) > T::zero() {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
