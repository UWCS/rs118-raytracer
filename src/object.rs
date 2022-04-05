use derive_more::Constructor;

use crate::{ray::Ray, vector::Point};

//a sphere
#[derive(Debug, Constructor)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

//calculate ray-sphere intersection stuff
impl Sphere {
    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
