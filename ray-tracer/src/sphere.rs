use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec3::Point3;
use crate::velem::VElem;
use std::rc::Rc;

pub struct Sphere<T: VElem> {
    center: Point3<T>,
    radius: T,
    material: Rc<dyn Material<T>>,
}

impl<T: VElem> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T, material: impl Material<T> + 'static) -> Self {
        Self {
            center,
            radius: T::max(0.0.into(), radius),
            material: Rc::new(material),
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

        //quaratic equation coefficients
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&o_to_c);
        let c = o_to_c.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < T::zero() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root_1 = (h - sqrtd) / a;
        let root_2 = (h + sqrtd) / a;

        // Check if either root is in the acceptable range
        let (t_near, t_far) = if ray_t.contains(&root_1) {
            (root_1, root_2)
        } else if ray_t.contains(&root_2) {
            (root_2, root_2)
        } else {
            return None;
        };

        // Choose the closest root (t_near) that is valid and in range
        let root = if t_near < t_far { t_near } else { t_far };

        // Calculate the point of intersection and the normal at the hit point
        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;

        // Create the hit record and set the face normal
        let mut hit_r = HitRecord {
            t: root,
            p: hit_point,
            normal: outward_normal,
            front_facing: false,
            material: self.material.clone(),
        };
        hit_r.set_face_normal(ray, outward_normal);

        Some(hit_r)
    }
}

#[cfg(test)]
mod tests {
    type Color = crate::color::Color<f32>;
    type Ray = crate::ray::Ray<f32>;
    type Sphere = super::Sphere<f32>;
    type Vec3 = crate::vec3::Vec3<f32>;
    use crate::hittable::Hittable;
    use crate::lambertian::Lambertian;
    use std::rc::Rc;

    #[test]
    fn test_ray_intersecting_sphere() {
        let mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: mat,
        };
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let range = 0.0..=10.0;

        if let Some(hit) = sphere.hit(&ray, range) {
            assert!(hit.t >= 0.0 && hit.t <= 10.0);
        } else {
            panic!("Ray should intersect the sphere");
        }
    }

    #[test]
    fn test_ray_tangent_to_sphere() {
        let mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let sphere = Sphere {
            center: Vec3::new(0.0, 2.0, -5.0),
            radius: 1.0,
            material: mat, /* Material initialization */
        };
        let ray = Ray::new(
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 1.0, 0.0).unit_vector(),
        );
        let range = 0.0..=10.0;

        if let Some(hit) = sphere.hit(&ray, range) {
            assert_eq!(hit.t, 1.0 /* Expected t value */);
        } else {
            panic!("Ray should be tangent to the sphere");
        }
    }

    #[test]
    fn test_ray_misses_sphere() {
        let mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: mat, /* Material initialization */
        };
        let ray = Ray::new(Vec3::new(2.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let range = 0.0..=10.0;

        assert!(sphere.hit(&ray, range).is_none());
    }

    #[test]
    fn test_ray_origin_inside_sphere() {
        let mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: mat, /* Material initialization */
        };
        let ray = Ray::new(Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 0.0, -1.0));
        let range = 0.0..=10.0;

        if let Some(hit) = sphere.hit(&ray, range) {
            assert!(hit.t >= 0.0 && hit.t <= 10.0);
        } else {
            panic!("Ray should intersect the sphere when exiting");
        }
    }

    #[test]
    fn test_ray_parallel_to_surface() {
        let mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let sphere = Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: mat, /* Material initialization */
        };
        let ray = Ray::new(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let range = 0.0..=10.0;

        assert!(sphere.hit(&ray, range).is_none());
    }
}
