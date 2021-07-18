use std::{ops, usize};

#[derive(Debug, Clone, Copy)]
pub struct CVec<T, const N: usize>
where
    T: Copy,
{
    data: [T; N],
}

impl<T, const N: usize> CVec<T, N>
where
    T: Copy,
{
    pub fn size(&self) -> usize {
        N
    }
}

impl<T, const N: usize> From<[T; N]> for CVec<T, N>
where
    T: Copy,
{
    fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: ops::AddAssign + ops::Mul<Output = T> + num_traits::Zero + Copy,
{
    pub fn length_squared(&self) -> T {
        let mut res = T::zero();
        for val in self.data {
            res += val * val;
        }
        res
    }
}

impl<T, const N: usize> CVec<T, N>
where
    T: ops::AddAssign + ops::Mul<Output = T> + From<f64> + Into<f64> + num_traits::Zero + Copy,
{
    pub fn length(&self) -> T {
        let l: f64 = self.length_squared().into();

        l.sqrt().into()
    }
}

impl<T, const N: usize> ops::Add for CVec<T, N>
where
    T: num_traits::Zero + ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut next = [T::zero(); N];

        for i in 0..self.data.len() {
            next[i] = self.data[i] + rhs.data[i];
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Sub for CVec<T, N>
where
    T: num_traits::Zero + ops::Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut next = [T::zero(); N];

        for i in 0..self.data.len() {
            next[i] = self.data[i] - rhs.data[i];
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Mul<Self> for CVec<T, N>
where
    T: num_traits::One + ops::Mul<T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut next = [T::one(); N];

        for i in 0..self.data.len() {
            next[i] = self.data[i] * rhs.data[i];
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Mul<T> for CVec<T, N>
where
    T: num_traits::One + ops::Mul<T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut next = [T::one(); N];

        for i in 0..self.data.len() {
            next[i] = self.data[i] * rhs;
        }

        next.into()
    }
}

impl<T, const N: usize> ops::Div<T> for CVec<T, N>
where
    T: num_traits::One + ops::Mul<T> + ops::Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self * (T::one() / rhs)
    }
}

impl<T, const N: usize> ops::AddAssign for CVec<T, N>
where
    T: ops::AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<T, const N: usize> ops::MulAssign<T> for CVec<T, N>
where
    T: ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for v in self.data.as_mut() {
            *v *= rhs;
        }
    }
}

impl<T, const N: usize> ops::DivAssign<T> for CVec<T, N>
where
    T: num_traits::One + ops::Div<Output = T> + ops::MulAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self *= T::one() / rhs;
    }
}

pub fn dot<T, const N: usize>(l: CVec<T, N>, r: CVec<T, N>) -> CVec<T, N>
where
    T: num_traits::Zero + ops::Mul<Output = T> + Copy,
{
    let mut next = [T::zero(); N];

    for i in 0..l.data.len() {
        next[i] = l.data[i] * r.data[i];
    }

    next.into()
}

pub type Vec3<T> = CVec<T, 3>;
pub type Color<T> = Vec3<T>;
pub type Point<T> = Vec3<T>;

impl<T> Vec3<T>
where
    T: Copy,
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

pub fn cross<T>(l: Vec3<T>, r: Vec3<T>) -> Vec3<T>
where
    T: num_traits::Zero + ops::Mul<Output = T> + ops::Sub<Output = T> + Copy,
{
    [
        l.data[1] * r.data[2] - l.data[2] * r.data[1],
        l.data[2] * r.data[0] - l.data[0] * r.data[2],
        l.data[0] * r.data[1] - l.data[1] * r.data[0],
    ]
    .into()
}

pub fn unit_vector<T>(v: Vec3<T>) -> Vec3<T>
where
    T: ops::AddAssign
        + ops::Mul<Output = T>
        + From<f64>
        + Into<f64>
        + num_traits::Zero
        + num_traits::One
        + ops::Div<Output = T>
        + ops::MulAssign<T>
        + Copy,
{
    let l = v.length();
    v / l
}
