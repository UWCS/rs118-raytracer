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
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            None
        } else {
            Some((-b - d.sqrt()) / (a * 2.0))
        }
    }
}
