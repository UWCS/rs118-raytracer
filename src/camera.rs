use crate::{ray::Ray, v, Point, Vec3};

pub struct Camera {
    origin: Point,
    top_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin: Point = v!(0, 0, 0);
        let horizontal = v!(viewport_width, 0, 0);
        let vertical = v!(0, -viewport_height, 0);
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
        let px_position = self.top_left + u * self.horizontal + v * self.vertical;
        Ray::new(self.origin, px_position - self.origin)
    }
}
