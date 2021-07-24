use std::{
    ops::{self, Neg},
    usize,
};

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CVec<T, const N: usize>
where
    T: Copy + Default,
{
    data: [T; N],
}

impl<T, const N: usize> Default for CVec<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            data: [Default::default(); N],
        }
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: Copy + Default,
{
    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        N
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: num_traits::NumRef + Neg<Output = T> + PartialOrd + SampleUniform + Default + Copy,
{
    pub fn random_range<R>(range: R) -> Self
    where
        R: SampleRange<T> + Clone,
    {
        let mut rng = rand::thread_rng();
        let mut data = [T::one(); N];

        for entry in data.as_mut() {
            *entry = rng.gen_range(range.clone());
        }

        Self { data }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-T::one()..T::one());

            if p.length_squared() < T::one() {
                return p;
            }
        }
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: num_traits::NumRef + Neg<Output = T> + PartialOrd + From<f64> + Default + Copy,
{
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        let s: T = S.into();

        let abs = |v: T| {
            if v >= T::zero() {
                v
            } else {
                -v
            }
        };

        let mut res = true;

        for d in self.data.iter() {
            res &= abs(*d) < s;

            if !res {
                return res;
            }
        }

        res
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: num_traits::NumRef + From<f64> + Into<f64> + Copy + Default,
{
    pub fn unit_vector(&self) -> Self {
        let l = self.length();
        *self / l
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: num_traits::NumRef
        + Neg<Output = T>
        + std::cmp::PartialOrd
        + From<f64>
        + Into<f64>
        + SampleUniform
        + Default
        + Copy,
{
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(&self) -> Self {
        let in_unit = Self::random_in_unit_sphere();
        if dot(in_unit, *self) > T::zero() {
            in_unit
        } else {
            in_unit * -T::one()
        }
    }
}
impl<T, const N: usize> From<[T; N]> for CVec<T, N>
where
    T: Copy + Default,
{
    fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: num_traits::NumRef + Default + Copy,
{
    pub fn length_squared(&self) -> T {
        let mut res = T::zero();
        for val in self.data {
            res = res + val * val;
        }
        res
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: num_traits::NumRef + From<f64> + Into<f64> + Default + Copy,
{
    pub fn length(&self) -> T {
        let l: f64 = self.length_squared().into();

        l.sqrt().into()
    }
}

impl<T, const N: usize> ops::Add for CVec<T, N>
where
    T: num_traits::NumRef + Copy + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut next = [T::zero(); N];

        for i in 0..self.len() {
            next[i] = self.data[i] + rhs.data[i];
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Sub for CVec<T, N>
where
    T: num_traits::NumRef + Default + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut next = [T::zero(); N];

        for i in 0..self.len() {
            next[i] = self.data[i] - rhs.data[i];
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Neg for CVec<T, N>
where
    T: num_traits::NumRef + Neg<Output = T> + Default + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -T::one()
    }
}

impl<T, const N: usize> ops::Mul<Self> for CVec<T, N>
where
    T: num_traits::NumRef + Default + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut next = [T::one(); N];

        for i in 0..self.len() {
            next[i] = self.data[i] * rhs.data[i];
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Mul<T> for CVec<T, N>
where
    T: num_traits::NumRef + Default + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut next = [T::one(); N];

        for i in 0..self.len() {
            next[i] = self.data[i] * rhs;
        }

        next.into()
    }
}

macro_rules! Muls {
     ($($e:ty),+) => {
         $(
             impl<const N: usize> ops::Mul<CVec<$e, N>> for $e
         {
             type Output = CVec<$e, N>;

             fn mul(self, rhs: CVec<$e, N>) -> Self::Output {
                 rhs * self
             }
         }
     )+
     };
 }

Muls!(usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl<T, const N: usize> ops::Div<T> for CVec<T, N>
where
    T: num_traits::NumRef + Default + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self * (T::one() / rhs)
    }
}

impl<T, const N: usize> ops::AddAssign for CVec<T, N>
where
    T: ops::AddAssign + Default + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.len() {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T, const N: usize> ops::MulAssign<T> for CVec<T, N>
where
    T: ops::MulAssign + Default + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for v in self.data.as_mut() {
            *v *= rhs;
        }
    }
}

impl<T, const N: usize> ops::DivAssign<T> for CVec<T, N>
where
    T: num_traits::NumAssignRef + Default + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self *= T::one() / rhs;
    }
}

pub fn dot<T, const N: usize>(l: CVec<T, N>, r: CVec<T, N>) -> T
where
    T: num_traits::NumRef + Default + Copy,
{
    let mut res = T::zero();
    for i in 0..l.len() {
        res = res + l.data[i] * r.data[i];
    }

    res
}

pub fn reflect<T, const N: usize>(v: CVec<T, N>, n: CVec<T, N>) -> CVec<T, N>
where
    T: num_traits::NumRef + From<f64> + Default + Copy,
{
    v - n * T::from(2.0) * dot(v, n)
}

pub fn refract<T, const N: usize>(uv: CVec<T, N>, n: CVec<T, N>, etai_over_etat: f64) -> CVec<T, N>
where
    T: num_traits::NumRef
        + From<f64>
        + ops::Mul<CVec<T, N>, Output = CVec<T, N>>
        + PartialOrd
        + Into<f64>
        + Neg<Output = T>
        + Default
        + Copy,
    f64: Into<T>,
{
    let cos_theta = dot(-uv, n);

    let cos_theta = if cos_theta < T::one() {
        cos_theta
    } else {
        T::one()
    };

    let r_out_perp = etai_over_etat.into() * (uv + cos_theta * n);
    let mut t: f64 = (T::one() - r_out_perp.length_squared()).into();
    t = -(t.abs().sqrt());
    let t: T = t.into();
    let r_out_parallel = t * n;

    r_out_perp + r_out_parallel
}

pub type Vec3<T> = CVec<T, 3>;
pub type Color<T> = Vec3<T>;
pub type Point<T> = Vec3<T>;

impl<T> Vec3<T>
where
    T: Copy + Default,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        [x, y, z].into()
    }

    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }

    pub fn z(&self) -> T {
        self.data[2]
    }
}

impl<T> Vec3<T>
where
    T: num_traits::NumRef + Neg<Output = T> + PartialOrd + SampleUniform + Default + Copy,
{
    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut p = Self::random_range(-T::one()..T::one());
            p.data[2] = T::zero();

            if p.length_squared() < T::one() {
                return p;
            }
        }
    }
}
impl<T> Vec3<T>
where
    T: num_traits::NumAssignRef + Default + Copy,
{
    pub fn cross(&self, rhs: &Self) -> Vec3<T> {
        [
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ]
        .into()
    }
}

pub fn cross<T>(l: &Vec3<T>, r: &Vec3<T>) -> Vec3<T>
where
    T: num_traits::NumAssignRef + Default + Copy,
{
    l.cross(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    fn setup() -> (CVec<f64, 5>, CVec<f64, 5>) {
        (
            [0.1, 0.2, 0.3, 0.4, 0.5].into(),
            [0.0, 0.1, 0.2, 0.3, 0.4].into(),
        )
    }

    #[test]
    fn test_into() {
        let res = "CVec { data: [0.1, 0.2, 0.3, 0.4, 0.5] }";
        let (v, _) = setup();
        let mut s = String::new();
        write!(&mut s, "{:?}", v).unwrap();

        assert_eq!(res, s);
    }

    #[test]
    fn test_size() {
        let (v, _) = setup();
        assert_eq!(v.len(), 5)
    }

    #[test]
    fn test_lenght_squared() {
        let (v, _) = setup();
        let r = v.length_squared();
        assert_eq!(0.55, r);
    }

    #[test]
    fn test_lenght() {
        let (v, _) = setup();
        let r = v.length();
        assert_eq!(0.55f64.sqrt(), r);
    }

    #[test]
    fn test_add() {
        let (v, l) = setup();
        let r: CVec<f64, 5> = [0.1 + 0.0, 0.2 + 0.1, 0.3 + 0.2, 0.4 + 0.3, 0.5 + 0.4].into();
        assert_eq!(v + l, r);
    }

    #[test]
    fn test_sub() {
        let (v, l) = setup();
        let r: CVec<f64, 5> = [0.1 - 0.0, 0.2 - 0.1, 0.3 - 0.2, 0.4 - 0.3, 0.5 - 0.4].into();
        assert_eq!(v - l, r);
    }

    #[test]
    fn test_mul_self() {
        let (v, l) = setup();
        let r: CVec<f64, 5> = [0.1 * 0.0, 0.2 * 0.1, 0.3 * 0.2, 0.4 * 0.3, 0.5 * 0.4].into();

        assert_eq!(v * l, r);
    }

    #[test]
    fn test_mul_f64() {
        let (v, _) = setup();
        let l = 1.2;
        let r: CVec<f64, 5> = [0.1 * l, 0.2 * l, 0.3 * l, 0.4 * l, 0.5 * l].into();

        assert_eq!(v * l, r);
    }

    #[test]
    fn test_div() {
        let (v, _) = setup();
        let ll = 1.2;
        let l = 1.0 / ll;
        let r: CVec<f64, 5> = [0.1 * l, 0.2 * l, 0.3 * l, 0.4 * l, 0.5 * l].into();

        assert_eq!(v / ll, r);
    }

    #[test]
    fn test_add_assign() {
        let (mut v, l) = setup();
        let r: CVec<f64, 5> = [0.1 + 0.0, 0.2 + 0.1, 0.3 + 0.2, 0.4 + 0.3, 0.5 + 0.4].into();
        v += l;
        assert_eq!(v, r);
    }

    #[test]
    fn test_mul_assign_f64() {
        let (mut v, _) = setup();
        let l = 1.2;
        let r: CVec<f64, 5> = [0.1 * l, 0.2 * l, 0.3 * l, 0.4 * l, 0.5 * l].into();

        v *= l;

        assert_eq!(v, r);
    }

    #[test]
    fn test_div_assign() {
        let (mut v, _) = setup();
        let ll = 1.2;
        let l = 1.0 / ll;
        let r: CVec<f64, 5> = [0.1 * l, 0.2 * l, 0.3 * l, 0.4 * l, 0.5 * l].into();

        v /= ll;

        assert_eq!(v, r);
    }

    #[test]
    fn test_dot() {
        let (v, l) = setup();
        let r = 0.1 * 0.0 + 0.2 * 0.1 + 0.3 * 0.2 + 0.4 * 0.3 + 0.5 * 0.4;
        assert_eq!(dot(v, l), r);
    }

    fn setup_vec3() -> (Vec3<f64>, Vec3<f64>) {
        (Vec3::new(0.1, 0.2, 0.3), Vec3::new(0.0, 0.1, 0.2))
    }

    #[test]
    fn test_cross() {
        let (v, l) = setup_vec3();
        let r: Vec3<f64> = [
            0.2 * 0.2 - 0.3 * 0.1,
            0.3 * 0.0 - 0.1 * 0.2,
            0.1 * 0.1 - 0.2 * 0.0,
        ]
        .into();
        assert_eq!(v.cross(&l), r);
        assert_eq!(cross(&v, &l), r);
    }
}
