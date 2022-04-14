use derive_more::Constructor;

use crate::{
    object::Hit,
    ray::Ray,
    vector::{Colour, Vec3},
};

#[derive(Debug)]
pub struct Reflection {
    pub ray: Ray,
    pub colour_attenuation: Colour,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Debug, Constructor)]
pub struct Lambertian(Colour);

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Reflection> {
        //calculate reflected ray
        let mut scatter_direction = hit.normal + Vec3::rand_unit();

        //account for possible zero direction
        if scatter_direction.is_zero() {
            scatter_direction = hit.normal;
        }

        let reflected_ray = Ray::new(hit.impact_point, scatter_direction);

        //return it, along with the colour attenuation of it for this material
        Some(Reflection {
            ray: reflected_ray,
            colour_attenuation: self.0,
        })
    }
}
