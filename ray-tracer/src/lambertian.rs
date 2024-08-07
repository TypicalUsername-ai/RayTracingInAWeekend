use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::velem::VElem;

pub struct Albertian<T: VElem> {
    albedo: Color<T>,
}

impl<T: VElem> Albertian<T> {
    pub fn new(albedo: Color<T>) -> Self {
        Self { albedo }
    }
}

impl<T: VElem> Material<T> for Albertian<T> {
    fn scatter(
        &self,
        ray_in: crate::ray::Ray<T>,
        hit: crate::hittable::HitRecord<T>,
        attenuation: Color<T>,
    ) -> crate::ray::Ray<T> {
        let scatter_dir = hit.normal + Vec3::random_unit_vec();
        Ray::new(hit.p, scatter_dir)
    }
}
