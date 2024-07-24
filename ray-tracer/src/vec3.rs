use crate::velem::VElem;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T: VElem> {
    xyz: [T; 3],
}

pub type Point3<T> = Vec3<T>;

impl<T> Vec3<T>
where
    T: VElem,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { xyz: [x, y, z] }
    }

    #[inline]
    pub fn x(&self) -> T {
        self.xyz[0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self.xyz[1]
    }

    #[inline]
    pub fn z(&self) -> T {
        self.xyz[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3::new(
            self.xyz[1] * rhs.xyz[2] - self.xyz[2] * rhs.xyz[1],
            self.xyz[2] * rhs.xyz[0] - self.xyz[0] * rhs.xyz[2],
            self.xyz[0] * rhs.xyz[1] - self.xyz[1] * rhs.xyz[0],
        )
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.xyz[0] * rhs.xyz[0] + self.xyz[1] * rhs.xyz[1] + self.xyz[2] * rhs.xyz[2]
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

#[cfg(test)]
mod impl_tests {

    use super::*;

    #[test]
    fn new_xyz_tests() {
        let v = Vec3::new(1.0, 2.2, 3.3);

        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.2);
        assert_eq!(v.z(), 3.3);
    }

    #[test]
    fn cross_product() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        let rhs = Vec3::from([4.0, 5.0, 6.0]);
        let cross = v.cross(&rhs);
        assert_eq!(cross.xyz, [-3.0, 6.0, -3.0])
    }

    #[test]
    fn length_tests() {
        let v = Vec3::from([0.0, -3.0, 4.0]);
        assert_eq!(v.length_squared(), 25.0);
        assert_eq!(v.length(), 5.0)
    }

    #[test]
    fn unit_vector() {
        let v = Vec3::from([0.0, -3.0, 4.0]);
        assert_eq!(v.unit_vector().xyz, [0.0, -0.6, 0.8]);
    }
}

impl<T: VElem + Default> Default for Vec3<T> {
    fn default() -> Self {
        Self {
            xyz: [T::default(); 3],
        }
    }
}

#[cfg(test)]
mod default_test {

    use super::*;

    #[test]
    fn default_test() {
        let v = Vec3::<f32>::default();
        assert_eq!(v.xyz, [0.0; 3])
    }
}

impl<T, F> From<F> for Vec3<T>
where
    T: VElem,
    F: Into<[T; 3]> + Sized,
{
    fn from(value: F) -> Self {
        Self { xyz: value.into() }
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

impl<T: VElem> ops::Index<usize> for Vec3<T> {
    type Output = T;

    /// this WILL panic!() if index > 2
    fn index(&self, index: usize) -> &Self::Output {
        &self.xyz[index]
    }
}

#[cfg(test)]
mod index_tests {

    use super::*;

    #[test]
    fn good_access() {
        let v = Vec3::from([0.0, 0.2, 0.4]);
        assert_eq!(v[0], 0.0);
        assert_eq!(v[1], 0.2);
        assert_eq!(v[2], 0.4);
    }

    #[test]
    #[should_panic]
    fn bad_access() {
        let v = Vec3::from([0.0, 0.2, 0.4]);
        assert_eq!(v[4], 0.0);
    }
}

impl<T: VElem> ops::IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.xyz[index]
    }
}

#[cfg(test)]
mod indexmut_tests {

    use super::*;

    #[test]
    fn good_access() {
        let mut v = Vec3::from([0.0, 0.2, 0.4]);
        v[0] = 2.0;
        v[1] = 3.3;
        v[2] = -22.0;
        assert_eq!(v[0], 2.0);
        assert_eq!(v[1], 3.3);
        assert_eq!(v[2], -22.0);
    }

    #[test]
    #[should_panic]
    fn bad_access() {
        let mut v = Vec3::from([0.0, 0.2, 0.4]);
        v[3] = -33.0;
    }
}

impl<T: VElem> ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            xyz: [
                self.xyz[0] + rhs.xyz[0],
                self.xyz[1] + rhs.xyz[1],
                self.xyz[2] + rhs.xyz[2],
            ],
        }
    }
}

#[cfg(test)]
mod add_tests {

    use super::*;

    #[test]
    fn base_add() {
        let v = Vec3::from([0.0, 0.1, 0.2]);
        let rhs = Vec3::from([0.1, 0.3, 0.5]);
        let sum = v + rhs;
        assert_eq!(sum.xyz, [0.1, 0.4, 0.7]);
    }

    #[test]
    fn overflow_add() {
        let v = Vec3::from([0.0, 0.1, f32::MAX]);
        let rhs = Vec3::from([0.1, 0.3, 0.5]);
        let sum = v + rhs;
        assert_eq!(sum.xyz, [0.1, 0.4, f32::MAX])
    }
}

impl<T: VElem> ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.xyz = [
            self.xyz[0] + rhs.xyz[0],
            self.xyz[1] + rhs.xyz[1],
            self.xyz[2] + rhs.xyz[2],
        ];
    }
}

#[cfg(test)]
mod addasign_tests {
    use super::*;

    #[test]
    fn base_add() {
        let mut v = Vec3::from([0.0, 0.1, 0.2]);
        let rhs = Vec3::from([0.1, 0.3, 0.5]);
        v += rhs;
        assert_eq!(v.xyz, [0.1, 0.4, 0.7]);
    }

    #[test]
    fn overflow_add() {
        let mut v = Vec3::from([0.0, 0.1, f32::MAX]);
        let rhs = Vec3::from([0.1, 0.3, 13.5]);
        v += rhs;
        assert_eq!(v.xyz, [0.1, 0.4, f32::MAX])
    }
}

impl<T: VElem> ops::Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            xyz: [
                self.xyz[0] - rhs.xyz[0],
                self.xyz[1] - rhs.xyz[1],
                self.xyz[2] - rhs.xyz[2],
            ],
        }
    }
}

#[cfg(test)]
mod sub_tests {

    use super::*;

    #[test]
    fn base_sub() {
        let v = Vec3::from([0.0, 1.0, 0.5]);
        let rhs = Vec3::from([0.1, 0.3, 13.5]);
        let sum = v - rhs;
        assert_eq!(sum.xyz, [-0.1, 0.7, -13.0])
    }

    #[test]
    fn overflow_sub() {
        let v = Vec3::from([0.0, 1.0, f32::MIN]);
        let rhs = Vec3::from([0.1, 0.3, 13.5]);
        let sum = v - rhs;
        assert_eq!(sum.xyz, [-0.1, 0.7, f32::MIN])
    }
}

impl<T: VElem> ops::SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.xyz = [
            self.xyz[0] - rhs.xyz[0],
            self.xyz[1] - rhs.xyz[1],
            self.xyz[2] - rhs.xyz[2],
        ];
    }
}

#[cfg(test)]
mod subassign_tests {

    use super::*;

    #[test]
    fn base_sub() {
        let mut v = Vec3::from([0.0, 1.0, 0.5]);
        let rhs = Vec3::from([0.1, 0.3, 13.5]);
        v -= rhs;
        assert_eq!(v.xyz, [-0.1, 0.7, -13.0])
    }

    #[test]
    fn overflow_sub() {
        let mut v = Vec3::from([0.0, 1.0, f32::MIN]);
        let rhs = Vec3::from([0.1, 0.3, 13.5]);
        v -= rhs;
        assert_eq!(v.xyz, [-0.1, 0.7, f32::MIN])
    }
}

impl<T: VElem> ops::Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            xyz: [
                self.xyz[0] * rhs.xyz[0],
                self.xyz[1] * rhs.xyz[1],
                self.xyz[2] * rhs.xyz[2],
            ],
        }
    }
}

impl<T: VElem> ops::Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            xyz: [self.xyz[0] * rhs, self.xyz[1] * rhs, self.xyz[2] * rhs],
        }
    }
}

#[cfg(test)]
mod mul_tests {

    use super::*;

    #[test]
    fn basic_mult() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        let rhs = Vec3::from([2.0, 0.0, 3.0]);
        let res = v * rhs;
        assert_eq!(res.xyz, [2.0, 0.0, 9.0])
    }

    #[test]
    fn nan_mult() {
        let v = Vec3::from([f32::NEG_INFINITY, 0.0, 0.0]);
        let rhs = Vec3::from([f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY]);
        let res = v * rhs;
        assert_eq!(res.x(), f32::INFINITY);
        assert!(res.y().is_nan());
        assert!(res.z().is_nan());
    }

    #[test]
    fn scalar_mul() {
        let v = Vec3::from([1.0, f32::INFINITY, f32::NEG_INFINITY]);
        let res = v * -2.0;
        assert_eq!(res.xyz, [-2.0, f32::NEG_INFINITY, f32::INFINITY])
    }

    #[test]
    fn scalar_nan_mul() {
        let v = Vec3::from([1.0, -2.0, 0.0]);
        let res = v * f32::NAN;
        assert!(res.x().is_nan());
        assert!(res.y().is_nan());
        assert!(res.z().is_nan());
    }
}

impl<T: VElem> ops::MulAssign for Vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.xyz = [
            self.xyz[0] * rhs.xyz[0],
            self.xyz[1] * rhs.xyz[1],
            self.xyz[2] * rhs.xyz[2],
        ];
    }
}

impl<T: VElem> ops::MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.xyz = [self.xyz[0] * rhs, self.xyz[1] * rhs, self.xyz[2] * rhs];
    }
}
#[cfg(test)]
mod mulassign_tests {

    use super::*;

    #[test]
    fn basic_mult() {
        let mut v = Vec3::from([1.0, 2.0, 3.0]);
        let rhs = Vec3::from([2.0, 0.0, 3.0]);
        v *= rhs;
        assert_eq!(v.xyz, [2.0, 0.0, 9.0])
    }

    #[test]
    fn nan_mult() {
        let mut v = Vec3::from([f32::NEG_INFINITY, 0.0, 0.0]);
        let rhs = Vec3::from([f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY]);
        v *= rhs;
        assert_eq!(v.x(), f32::INFINITY);
        assert!(v.y().is_nan());
        assert!(v.z().is_nan());
    }

    #[test]
    fn scalar_mul() {
        let mut v = Vec3::from([1.0, f32::INFINITY, f32::NEG_INFINITY]);
        v *= -2.0;
        assert_eq!(v.xyz, [-2.0, f32::NEG_INFINITY, f32::INFINITY])
    }

    #[test]
    fn scalar_nan_mul() {
        let mut v = Vec3::from([1.0, -2.0, 0.0]);
        v *= f32::NAN;
        assert!(v.x().is_nan());
        assert!(v.y().is_nan());
        assert!(v.z().is_nan());
    }
}

impl<T: VElem> ops::Div for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            xyz: [
                self.xyz[0] / rhs.xyz[0],
                self.xyz[1] / rhs.xyz[1],
                self.xyz[2] / rhs.xyz[2],
            ],
        }
    }
}

impl<T: VElem> ops::Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            xyz: [self.xyz[0] / rhs, self.xyz[1] / rhs, self.xyz[2] / rhs],
        }
    }
}

#[cfg(test)]
mod div_tests {

    use super::*;

    #[test]
    fn basic_div() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        let rhs = Vec3::from([2.0, 0.0, -3.0]);
        let res = v / rhs;
        assert_eq!(res.x(), 0.5);
        assert_eq!(res.y(), f32::INFINITY);
        assert_eq!(res.z(), -1.0);
    }

    #[test]
    fn nan_div() {
        let v = Vec3::from([f32::NEG_INFINITY, 0.0, f32::INFINITY]);
        let rhs = Vec3::from([f32::NEG_INFINITY, f32::INFINITY, -0.0]);
        let res = v / rhs;
        assert!(res.x().is_nan());
        assert_eq!(res.y(), 0.0);
        assert_eq!(res.z(), f32::NEG_INFINITY);
    }

    #[test]
    fn scalar_div() {
        let v = Vec3::from([1.0, f32::INFINITY, f32::NEG_INFINITY]);
        let res = v / -2.0;
        assert_eq!(res.xyz, [-0.5, f32::NEG_INFINITY, f32::INFINITY])
    }

    #[test]
    fn scalar_nan_div() {
        let v = Vec3::from([1.0, f32::INFINITY, f32::NEG_INFINITY]);
        let res = v / f32::NAN;
        assert!(res.x().is_nan());
        assert!(res.y().is_nan());
        assert!(res.z().is_nan());
    }
}

impl<T: VElem> ops::DivAssign for Vec3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.xyz = [
            self.xyz[0] / rhs.xyz[0],
            self.xyz[1] / rhs.xyz[1],
            self.xyz[2] / rhs.xyz[2],
        ];
    }
}

impl<T: VElem> ops::DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.xyz = [self.xyz[0] / rhs, self.xyz[1] / rhs, self.xyz[2] / rhs];
    }
}

#[cfg(test)]
mod divassign_tests {

    use super::*;

    #[test]
    fn basic_div() {
        let mut v = Vec3::from([1.0, 2.0, 3.0]);
        let rhs = Vec3::from([2.0, 0.0, -3.0]);
        v /= rhs;
        assert_eq!(v.x(), 0.5);
        assert_eq!(v.y(), f32::INFINITY);
        assert_eq!(v.z(), -1.0);
    }

    #[test]
    fn nan_div() {
        let mut v = Vec3::from([f32::NEG_INFINITY, 0.0, f32::INFINITY]);
        let rhs = Vec3::from([f32::NEG_INFINITY, f32::INFINITY, -0.0]);
        v /= rhs;
        assert!(v.x().is_nan());
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), f32::NEG_INFINITY);
    }

    #[test]
    fn scalar_div() {
        let mut v = Vec3::from([1.0, f32::INFINITY, f32::NEG_INFINITY]);
        v /= -2.0;
        assert_eq!(v.xyz, [-0.5, f32::NEG_INFINITY, f32::INFINITY])
    }

    #[test]
    fn scalar_nan_div() {
        let mut v = Vec3::from([1.0, f32::INFINITY, f32::NEG_INFINITY]);
        v /= f32::NAN;
        assert!(v.x().is_nan());
        assert!(v.y().is_nan());
        assert!(v.z().is_nan());
    }
}
