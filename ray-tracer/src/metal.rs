use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::velem::VElem;

pub struct Metal<T: VElem> {
    albedo: Color<T>,
}

impl<T: VElem> Metal<T> {
    fn new(albedo: Color<T>) -> Self {
        Self { albedo }
    }
}

impl<T: VElem> Material<T> for Metal<T> {
    fn scatter(
        &self,
        ray_in: crate::ray::Ray<T>,
        hit: crate::hittable::HitRecord<T>,
        attenuation: Color<T>,
    ) -> crate::ray::Ray<T> {
        let reflected = ray_in.direction().reflect(hit.normal);
        Ray::new(hit.p, reflected)
    }
}
