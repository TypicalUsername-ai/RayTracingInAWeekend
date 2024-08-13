use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::velem::VElem;

pub struct Dielectric<T: VElem> {
    refraction_index: T,
}

impl<T: VElem> Material<T> for Dielectric<T> {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray<T>,
        hit: &crate::hittable::HitRecord<T>,
    ) -> Option<(crate::ray::Ray<T>, crate::color::Color<T>)> {
        let attenuation = Color::from([T::one(); 3]);
        let ri = if hit.front_facing {
            T::one() / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = ray_in.direction().unit_vector();
        let refracted = unit_dir.refract(hit.normal, ri);

        Some((Ray::new(hit.p, refracted), attenuation))
    }
}
