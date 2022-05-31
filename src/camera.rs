use rand::prelude::Distribution;

use crate::{ray::Ray, v, Point, Vec3};

pub struct Camera {
    origin: Point,
    top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let view_height = 2.0 * h;
        let view_width = aspect_ratio * view_height;

        let w = (look_from - look_at).normalise();
        let u = vup.cross(&w).normalise();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = view_width * u * focus_distance;
        let vertical = -view_height * v * focus_distance;

        let top_left: Point = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            top_left,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand = random_in_unit_circle() * self.lens_radius;
        let origin = self.origin + self.u * rand.x + self.v * rand.y;

        let px_position = self.top_left + s * self.horizontal + t * self.vertical;

        //return the ray pointing at those pixels from camera origin
        Ray::new(origin, px_position - origin)
    }
}

fn random_in_unit_circle() -> Vec3 {
    //want random numbers -1 to 1
    let dist = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
    let mut rng = rand::thread_rng();
    loop {
        let v = v!(dist.sample(&mut rng), dist.sample(&mut rng), 0);
        //if the vector lies in the unit sphere
        if v.len() < 1.0 {
            //normalise so it lies *on* the sphere
            break v.normalise();
        }
    }
}
