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
