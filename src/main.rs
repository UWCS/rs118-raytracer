mod ray;
mod vector;

use image::RgbImage;
use ray::Ray;
use vector::{Point, Vec3};

fn main() {
    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;

    //camera and viewport
    let view_height = 2.0;
    let view_width = view_height * aspect_ratio;
    let focal_length = 1.0;

    //geometry
    let origin: Point = v!(0, 0, 0);
    let horizontal: Vec3 = v!(view_width, 0, 0); //horizontal size vector
    let vertical: Vec3 = v!(0, -view_height, 0); //vertical size vector, negated because we start in the top left and move *down* when rendering
    let top_left: Point = origin - horizontal / 2.0 - vertical / 2.0 - v!(0, 0, focal_length); //the position of the top left corner of our imgae

    let mut buffer = RgbImage::new(img_width, img_height);

    for (i, j, px) in buffer.enumerate_pixels_mut() {
        //pixel coordinates as scalars from 0.0 <= t <= 1.0
        let u = i as f64 / (img_width - 1) as f64;
        let v = j as f64 / (img_height - 1) as f64;

        //the direction of the ray
        //start at top left, then go horizontally scaled by u and vertically by v
        let ray_direction: Vec3 = top_left + u * horizontal + v * vertical - origin;

        //save pixel colour to buffer
        *px = ray::colour(&Ray::new(origin, ray_direction)).to_rgb();
    }
    buffer.save("render.png").expect("Could not save image");
}
