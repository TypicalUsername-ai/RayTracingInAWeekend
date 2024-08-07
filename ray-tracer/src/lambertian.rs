use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::velem::VElem;
use num_traits::Zero;

pub struct Lambertian<T: VElem> {
    albedo: Color<T>,
}

impl<T: VElem> Lambertian<T> {
    pub fn new(albedo: Color<T>) -> Self {
        Self { albedo }
    }
}

impl<T: VElem> Material<T> for Lambertian<T> {
    fn scatter(
        &self,
        _ray_in: &crate::ray::Ray<T>,
        hit: &crate::hittable::HitRecord<T>,
    ) -> (crate::ray::Ray<T>, crate::color::Color<T>) {
        let scatter_dir = hit.normal + Vec3::random_unit_vec();
        // hopefully catch degen scatter dirs
        let scattered = if scatter_dir.is_zero() {
            Ray::new(hit.p, hit.normal)
        } else {
            Ray::new(hit.p, scatter_dir)
        };
        (scattered, self.albedo)
    }
}
