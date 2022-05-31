use crate::{ray::Ray, Point, Vec3};

pub struct Camera {
    origin: Point,
    top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point, vup: Vec3, fov: f64, aspect_ratio: f64) -> Self {
        let theta = fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let view_height = 2.0 * h;
        let view_width = aspect_ratio * view_height;

        let w = (look_from - look_at).normalise();
        let u = vup.cross(&w).normalise();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = view_width * u;
        let vertical = -view_height * v;

        let top_left: Point = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            top_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        //calculate the pixel position from the top left using the offsets and size vectors
        let px_position = self.top_left + u * self.horizontal + v * self.vertical;

        //return the ray pointing at those pixels from camera origin
        Ray::new(self.origin, px_position - self.origin)
    }
}
