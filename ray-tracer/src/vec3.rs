#[derive(Debug)]
pub struct Vec3 {
    xyz: [f32; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { xyz: [0.0; 3] }
    }
}

#[cfg(test)]
mod default_test {
    use super::*;

    #[test]
    fn default_test() {
        let v = Vec3::default();
        assert_eq!(v.xyz, [0.0; 3])
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self { xyz: value }
    }
}
#[cfg(test)]
mod from_tests {

    use super::*;

    #[test]
    fn from_works() {
        let v = Vec3::from([0.0, 0.1, 0.2]);
        assert_eq!(v.xyz, [0.0, 0.1, 0.2])
    }
}
