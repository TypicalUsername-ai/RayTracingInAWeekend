use crate::vec3::Vec3;
use crate::velem::VElem;
use std::io::Write;

pub type Color<T> = Vec3<T>;

impl<T: VElem> Color<T> {
    #[inline]
    fn to_gamma(linear: T) -> T {
        if linear >= T::zero() {
            linear.sqrt()
        } else {
            T::zero()
        }
    }

    pub fn write_color<W>(self, out: &mut W) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        let intensity: std::ops::RangeInclusive<T> = 0.0.into()..=0.999.into();
        let r = Self::to_gamma(self.x().clamp(*intensity.start(), *intensity.end()));
        let g = Self::to_gamma(self.y().clamp(*intensity.start(), *intensity.end()));
        let b = Self::to_gamma(self.z().clamp(*intensity.start(), *intensity.end()));
        writeln!(
            out,
            "{} {} {}",
            (r * 256.0.into()).trunc(),
            (g * 256.0.into()).trunc(),
            (b * 256.0.into()).trunc(),
        )
    }
}

#[cfg(test)]
mod color_tests {

    use super::*;

    #[test]
    fn test_color() {
        let v = Color::from([1.0, 0.0, 4.0]);
        let mut s = Vec::new();
        v.unit_vector()
            .write_color(&mut s)
            .expect("Should write normally");
        let st = String::from_utf8(s).unwrap();
        assert_eq!(st, "126 0 252\n");
    }
}
