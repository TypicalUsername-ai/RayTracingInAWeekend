use std::io::{stderr, stdout, Write};
mod color;
mod ray;
mod vec3;
use color::Color;
use ray::Ray;
use vec3::Vec3;

fn main() {
    let width = 256;
    let height = 256;

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
