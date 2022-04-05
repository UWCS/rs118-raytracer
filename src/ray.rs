use derive_more::Constructor;

use crate::{
    v,
    vector::{Colour, Point, Vec3},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Constructor)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

pub fn colour(_: &Ray) -> Colour {
    v!(0, 1.0, 0)
}
