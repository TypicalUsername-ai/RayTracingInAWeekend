use crate::vec3::Vec3;
use std::io::Write;

type Color = Vec3;

impl Color {
    pub fn write_color<T>(self, out: &mut T) -> Result<(), std::io::Error>
    where
        T: Write,
    {
        let r = (self.x() * 255.999).round() as u8;
        let g = (self.y() * 255.999).round() as u8;
        let b = (self.z() * 255.999).round() as u8;
        writeln!(out, "{} {} {}", r, g, b)
    }
}

#[cfg(test)]
mod color_tests {

    use super::*;

    #[test]
    fn test_color() {
        let v = Color::from([3.0, 0.0, 4.0]);
        let mut s = Vec::new();
        v.unit_vector()
            .write_color(&mut s)
            .expect("Should write normally");
        let st = String::from_utf8(s).unwrap();
        assert_eq!(st, "154 0 205\n");
    }
}
