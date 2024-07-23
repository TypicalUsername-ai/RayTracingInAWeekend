use std::io::{stderr, stdout, Write};
mod color;
mod ray;
mod vec3;
mod velem;
use color::Color;

type Vec3 = vec3::Vec3<f32>;
type Ray = ray::Ray<f32>;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio).round() as i32;
    assert!(height > 1);

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f32 / height as f32);

    let mut lock = stdout().lock();
    let mut err = stderr().lock();
    write!(lock, "P3\n{width} {height}\n255\n").unwrap();
    for y in 0..height {
        write!(err, "\rLines remaining: {}  ", height - y).unwrap();
        let _ = err.flush();
        for x in 0..width {
            let pixel = Color::new(
                x as f32 / (width - 1) as f32,
                y as f32 / (height - 1) as f32,
                0.0,
            );
            pixel.write_color(&mut lock).unwrap()
        }
    }
    write!(err, "\r Done                          \n").unwrap();
}
