use std::io::{stderr, stdout, Write};
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod velem;

type Vec3 = vec3::Vec3<f32>;
type Ray = ray::Ray<f32>;
type Point = vec3::Point3<f32>;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio).round() as i32;
    assert!(height > 1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f32 / height as f32);
    let camera_center = Point::from([0.0; 3]);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    // veiwport_u x_min -> x_max
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    // viewport_v y_max -> y_min
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    // pixel_delta_u -> pixel step horizontal for the viewport
    let pixel_delta_u: Vec3 = viewport_u / width as f32;
    // pixel_delta_v -> pixel step vertical for the viewport
    let pixel_delta_v: Vec3 = viewport_v / height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    // as the loop doesn't follow the xyz coordinates in the system we need to calculate the actual
    // pixel 0 location
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // stdout writer lock
    let mut lock = stdout().lock();
    // stderr writer lock
    let mut err = stderr().lock();
    write!(lock, "P3\n{width} {height}\n255\n").unwrap();
    for y in 0..height {
        write!(err, "\rLines remaining: {}  ", height - y).unwrap();
        let _ = err.flush();
        for x in 0..width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let color = r.color();
            color.write_color(&mut lock).unwrap();
        }
    }
    write!(err, "\r Done                          \n").unwrap();
}
