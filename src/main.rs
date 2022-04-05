mod ray;
mod vector;

use image::{Rgb, RgbImage};
fn main() {
    let (width, height) = (600, 400);
    let mut buffer = RgbImage::new(width, height);
    for (i, j, px) in buffer.enumerate_pixels_mut() {
        let r = i as f64 / (width - 1) as f64;
        let g = j as f64 / (height - 1) as f64;
        let b = 0.25;

        *px = Rgb([r, g, b].map(|c| (c * 255.999) as u8))
    }
    buffer.save("render.png").expect("Could not save image");
}
