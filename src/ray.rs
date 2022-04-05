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
    //calculate a point along a ray based on the parameter t
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

pub fn colour(ray: &Ray) -> Colour {
    let direction = ray.direction.normalise();
    let t = 0.5 * (direction.normalise().y + 1.0); //scale from -1 < y < 1 to  0 < t < 1

    //two colours to blend
    let white: Colour = v!(1);
    let blue: Colour = v!(0.5, 0.7, 1);

    //blend
    blue * t + white * (1.0 - t)
}
