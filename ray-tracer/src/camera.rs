use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::velem::VElem;
use std::io::{stderr, stdout, Write};

pub struct Camera<T: VElem> {
    aspect_ratio: T,
    image_width: u64,
    image_height: u64,
    center: Point3<T>,
    pixel00_loc: Point3<T>,
    pixel_delta_u: Vec3<T>,
    pixel_delta_v: Vec3<T>,
}

impl<T: VElem> Camera<T> {
    pub fn new(aspect_ratio: (u8, u8), image_width: u64) -> Self {
        let aspect_ratio = aspect_ratio.0 as f32 / aspect_ratio.1 as f32;
        let image_height = (image_width as f32 / aspect_ratio) as u64;
        assert!(image_height > 1);

        // Camera
        let focal_length = 1.0;
        let viewport_height: T = 2.0.into();
        let viewport_width: T = viewport_height * (image_width as f32 / image_height as f32).into();
        let camera_center = Point3::<T>::default();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        // veiwport_u x_min -> x_max
        let viewport_u = Vec3::<T>::new(viewport_width.into(), T::zero(), T::zero());
        // viewport_v y_max -> y_min
        let viewport_v = Vec3::<T>::new(T::zero(), (-viewport_height).into(), T::zero());

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        // pixel_delta_u -> pixel step horizontal for the viewport
        let pixel_delta_u = viewport_u / Into::<T>::into(image_width as f32);
        // pixel_delta_v -> pixel step vertical for the viewport
        let pixel_delta_v = viewport_v / Into::<T>::into(image_height as f32);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
            - Vec3::new(0.0.into(), 0.0.into(), focal_length.into())
            - viewport_u / Into::<T>::into(2.0)
            - viewport_v / Into::<T>::into(2.0);

        // as the loop doesn't follow the xyz coordinates in the system we need to calculate the actual
        // pixel 0 location
        let pixel00_loc =
            viewport_upper_left + (pixel_delta_u + pixel_delta_v) * Into::<T>::into(0.5);

        Self {
            aspect_ratio: aspect_ratio.into(),
            image_width,
            image_height,
            center: camera_center,
            pixel_delta_v,
            pixel_delta_u,
            pixel00_loc,
        }
    }

    pub fn render(&self, world: &impl Hittable<T>) {
        // stdout writer lock
        let mut lock = stdout().lock();
        // stderr writer lock
        let mut err = stderr().lock();
        write!(
            lock,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();
        for y in 0..self.image_height {
            write!(err, "\rLines remaining: {}  ", self.image_height - y).unwrap();
            let _ = err.flush();
            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * Into::<T>::into(x as f32))
                    + (self.pixel_delta_v * Into::<T>::into(y as f32));
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let color = self.ray_color(&r, world);
                color.write_color(&mut lock).unwrap();
            }
        }
        write!(err, "\r Done                          \n").unwrap();
    }

    fn ray_color(&self, ray: &Ray<T>, world: &impl Hittable<T>) -> Color<T> {
        if let Some(hr) = world.hit(ray, 0.0.into()..=f32::MAX.into()) {
            return (hr.normal + Color::from([1.0.into(); 3])) * Into::<T>::into(0.5);
        }
        let unit_direction = ray.direction().unit_vector();
        let a: T = (unit_direction.y() + 1.0.into()) * 0.5.into();
        let mut result = Color::new(1.0.into(), 1.0.into(), 1.0.into());
        let scaler: T = (<T as From<f32>>::from(1.0) - a).into();
        result *= scaler;
        result += Color::new(0.5.into(), 0.7.into(), 1.0.into()) * a;
        result
    }
}
