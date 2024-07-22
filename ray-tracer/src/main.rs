use std::io::{stderr, stdout, Write};
mod vec3;

fn main() {
    let width = 256;
    let height = 256;

    let mut lock = stdout().lock();
    let mut err = stderr().lock();
    write!(lock, "P3\n{width} {height}\n255\n");
    for y in 0..height {
        write!(err, "\rLines remaining: {}  ", height - y);
        err.flush();
        for x in 0..width {
            let red = x as f32 / (width - 1) as f32;
            let green = y as f32 / (height - 1) as f32;
            let blue = 0f32;
            writeln!(
                lock,
                "{} {} {}",
                (red * 255.999) as u8,
                (green * 255.999) as u8,
                (blue * 255.999) as u8
            );
        }
    }
    write!(err, "\r Done                          \n");
}
