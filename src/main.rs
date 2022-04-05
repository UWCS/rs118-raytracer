mod ray;
mod vector;

use image::RgbImage;
use ray::Ray;
use vector::Vec3;

fn main() {
    //image
    let aspect_ratio = 16.0 / 9.0;
    let (width, height) = ((400.0 * aspect_ratio) as u32, 400);

    //camera
    let view_height = 2.0;
    let view_width = view_height * aspect_ratio;
    let focal_length = 1.0;

    //geometry
    let origin = v!(0, 0, 0);
    let horizontal = v!(view_width, 0, 0);
    let vertical = v!(0, -view_height, 0);
    let top_left = origin - horizontal / 2.0 - vertical / 2.0 - v!(0, 0, focal_length);

    let mut buffer = RgbImage::new(width, height);
    for (i, j, px) in buffer.enumerate_pixels_mut() {
        let u = i as f64 / (width - 1) as f64;
        let v = j as f64 / (height - 1) as f64;
        let ray_direction = top_left + u * horizontal + v * vertical - origin;

        *px = ray::colour(&Ray::new(origin, ray_direction)).to_rgb();
    }
    buffer.save("render.png").expect("Could not save image");
}
