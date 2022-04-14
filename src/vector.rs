use derive_more::{Add, Constructor, Div, Mul, Neg, Sub};
use std::ops::Mul;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Add, Div, Mul, Sub, Neg, Constructor)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! v {
    ($x:expr, $y: expr, $z: expr) => {
        Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
    ($x:expr) => {
        Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };
}

pub type Point = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalise(self) -> Self {
        self / self.len()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn rand_unit() -> Self {
        loop {
            //random f64 range 0-1, scale it -1 to 1
            let v = v!(rand::random::<f64>() * 2.0 - 1.0);

            //if the vector lies in the unit sphere
            if v.len() < 1.0 {
                //normalise so it lies *on* the sphere and is a unit vector
                break v.normalise();
            }
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn to_rgb(self) -> image::Rgb<u8> {
        image::Rgb(
            [self.x, self.y, self.z]
                .map(|c| c.sqrt())
                .map(|c| (c * 255.999) as u8),
        )
    }

    pub fn map<F>(self, mut f: F) -> Vec3
    where
        F: FnMut(f64) -> f64,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| x * self)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
