use std::rc::Rc;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod velem;

type Point = vec3::Point3<f32>;
type Camera = camera::Camera<f32>;

fn main() {
    let world: hittable_list::HittableList<f32> = vec![
        Rc::new(sphere::Sphere::new(Point::from([0.0, 0.0, -1.0]), 0.5)),
        Rc::new(sphere::Sphere::new(Point::from([0.0, -100.5, -1.0]), 100.0)),
    ];

    // image
    let aspect_ratio = (16, 9);
    let width = 400;
    let c = Camera::new(aspect_ratio, width, 100, 10);
    c.render(&world)
}
