use derive_more::Constructor;

use crate::{
    object::Hit,
    ray::Ray,
    v,
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

#[derive(Debug, Constructor)]
pub struct Metal {
    colour: Colour,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        //the reflected ray direction
        let reflection =
            reflect(incident_ray.direction, &hit.normal) + self.fuzz * Vec3::rand_unit();

        //the scattered ray
        let scattered = Ray::new(hit.impact_point, reflection);

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(Reflection {
                ray: scattered,
                colour_attenuation: self.colour,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Constructor)]
pub struct Dielectric(f64);

impl Material for Dielectric {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let ratio = if hit.front_face { 1.0 / self.0 } else { self.0 };
        let unit_direction = incident_ray.direction.normalise();

        let cos_theta = -unit_direction.dot(&hit.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let scatter_direction =
            if (ratio * sin_theta > 1.0) || reflectance(cos_theta, ratio) > rand::random() {
                reflect(unit_direction, &hit.normal)
            } else {
                refract(unit_direction, &hit.normal, ratio)
            };

        let out_ray = Ray::new(hit.impact_point, scatter_direction);
        Some(Reflection {
            ray: out_ray,
            colour_attenuation: v!(1),
        })
    }
}

fn reflect(v: Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * *normal
}

fn refract(incident: Vec3, normal: &Vec3, ratio: f64) -> Vec3 {
    let cos_theta = -incident.dot(normal);
    let r_out_perp = ratio * (incident + cos_theta * *normal);
    let r_out_par = -(1.0 - r_out_perp.dot(&r_out_perp)).abs().sqrt() * *normal;
    r_out_par + r_out_perp
}

fn reflectance(cos_theta: f64, n: f64) -> f64 {
    let r0 = f64::powi((1.0 - n) / (1.0 + n), 2);
    r0 + (1.0 - r0) * f64::powi(1.0 - cos_theta, 5)
}
