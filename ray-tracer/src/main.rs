use std::rc::Rc;
mod camera;
mod color;
mod dielectric;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod vec3;
mod velem;

type Point = vec3::Point3<f32>;
type Camera = camera::Camera<f32>;
type Color = color::Color<f32>;

fn main() {
    let ground = lambertian::Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center = lambertian::Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let left = metal::Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let right = metal::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let world: hittable_list::HittableList<f32> = vec![
        Rc::new(sphere::Sphere::new(
            Point::from([0.0, 0.0, -1.2]),
            0.5,
            center,
        )),
        Rc::new(sphere::Sphere::new(
            Point::from([-1.0, 0.0, -1.0]),
            0.5,
            left,
        )),
        Rc::new(sphere::Sphere::new(
            Point::from([1.0, 0.0, -1.0]),
            0.5,
            right,
        )),
        Rc::new(sphere::Sphere::new(
            Point::from([0.0, -100.5, -1.0]),
            100.0,
            ground,
        )),
    ];

    // image
    let aspect_ratio = (16, 9);
    let width = 400;
    let c = Camera::new(aspect_ratio, width, 100, 10);
    c.render(&world)
}
