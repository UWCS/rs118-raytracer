use derive_more::Constructor;

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

//Information about a ray-object intersection
pub struct Hit {
    pub impact_point: Point,
    pub normal: Vec3,
    pub paramater: f64,
    pub front_face: bool,
}

// Represents objects within the scene
pub trait Object {
    //determines if an object has been hit by a ray
    //returns the impace point, the surfac normal to the impact point, and the solution to the impact equation
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}
//a sphere
#[derive(Debug, Constructor)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        //calculate intersection
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;

        if d < 0.0 {
            return None;
        }

        //get the correct root, if one lies in the bounds
        let mut root = (-b - d.sqrt()) / (2.0 * a);
        if !(bounds.0..bounds.1).contains(&root) {
            root = (-b + d.sqrt()) / (2.0 * a);
            if !(bounds.0..bounds.1).contains(&root) {
                return None;
            }
        }

        let impact_point = ray.at(root);
        let normal = (impact_point - self.center) / self.radius;

        //make sure the normals always point outward from the sphere's surface
        //against the indicent ray
        let (normal, front_face) = if ray.direction.dot(&normal) > 0.0 {
            (-normal, false)
        } else {
            (normal, true)
        };

        Some(Hit {
            impact_point,
            normal,
            paramater: root,
            front_face,
        })
    }
}

//make a list of objects representing a scene an object we can impact, returning the impact of the closest one

pub type Scene = Vec<Box<dyn Object + Sync>>;

impl Object for Scene {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.iter()
            .filter_map(|o| o.hit(ray, bounds)) //filter out the ones that don't intersect
            .min_by(|h1, h2| h1.paramater.partial_cmp(&h2.paramater).unwrap()) //sort by smallest parameter, returning lowest
    }
}
