use derive_more::Constructor;

use crate::vector::{Point, Vec3};

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
