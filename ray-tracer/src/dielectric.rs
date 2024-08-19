use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::velem::VElem;
use rand::Rng;

pub struct Dielectric<T: VElem> {
    refraction_index: T,
}

impl<T: VElem> Dielectric<T> {
    pub fn new(refraction_index: T) -> Self {
        Self { refraction_index }
    }

    /// function for reflectance done using schlick's approximation
    fn reflectance(cos: T, refraction_index: T) -> T {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (T::one() - refraction_index) / (T::one() + refraction_index);
        r0 = r0 * r0;
        return r0 + (T::one() - r0) * (T::one() - cos).powf(5.0.into());
    }
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

        let cos_theta = T::min((-unit_dir).dot(&hit.normal), T::one());
        let sin_theta = T::sqrt(T::one() - cos_theta * cos_theta);

        let direction = if ri * sin_theta > T::one()
            || Self::reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..=1.0).into()
        {
            unit_dir.reflect(hit.normal)
        } else {
            unit_dir.refract(hit.normal, ri)
        };

        Some((Ray::new(hit.p, direction), attenuation))
    }
}
