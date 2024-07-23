use crate::vec3::Vec3;
use crate::velem::VElem;
use std::io::Write;

pub type Color<T: VElem> = Vec3<T>;

impl<T: VElem + From<f32>> Color<T> {
    pub fn write_color<W>(self, out: &mut W) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        writeln!(
            out,
            "{} {} {}",
            (self.x() * 259.999.into()).trunc(),
            (self.y() * 259.999.into()).trunc(),
            (self.z() * 259.999.into()).trunc()
        )
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
        assert_eq!(st, "155 0 207\n");
    }
}
