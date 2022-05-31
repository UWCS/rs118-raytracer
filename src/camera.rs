use crate::{ray::Ray, v, Point, Vec3};

pub struct Camera {
    origin: Point,
    top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(fov: f64, aspect_ratio: f64) -> Self {
        let theta = fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let view_height = 2.0 * h;
        let view_width = aspect_ratio * view_height;

        let focal_length = 1.0;

        let origin: Point = v!(0, 0, 0);
        let horizontal = v!(view_width, 0, 0);
        let vertical = v!(0, -view_height, 0);
        //the top  left of our image is the origin, -1 away from the camera and up and right by half the height/width
        let top_left: Point = origin - horizontal / 2.0 - vertical / 2.0 - v!(0, 0, focal_length);

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
