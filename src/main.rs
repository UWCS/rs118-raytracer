use image::{Rgb, RgbImage};
fn main() {
    let (width, height) = (256, 256);
    let mut buffer = RgbImage::new(width, height);
    for (_, _, px) in buffer.enumerate_pixels_mut() {
        *px = Rgb([255, 0, 0]);
    }
    buffer.save("render.png").expect("Could not save image");
}
