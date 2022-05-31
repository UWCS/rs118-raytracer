mod camera;
mod material;
mod object;
mod ray;
mod vector;

use image::RgbImage;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use material::Lambertian;
use object::{Scene, Sphere};
use rayon::prelude::*;
use vector::{Point, Vec3};

use crate::material::{Dielectric, Metal};

fn main() {
    //image
    let aspect_ratio = 1.5;
    let img_width: u32 = 1200;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples: u32 = 100;
    let max_depth = 50;

    //camera struct
    let look_from = v!(13, 2, 3);
    let look_at = v!(0, 0, 0);
    let camera = camera::Camera::new(
        look_from,
        look_at,
        v!(0, 1, 0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    //create image buffer
    let mut buffer = RgbImage::new(img_width, img_height);

    //world
    let objects: Scene = random_scene();

    println!("Rendering Scene...");
    let bar = ProgressBar::new((img_width * img_height) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#>-")
            .on_finish(ProgressFinish::WithMessage("-- Done!".into())),
    );

    buffer
        .enumerate_pixels_mut()
        .par_bridge() // Rayon go brrrrrrr
        .progress_with(bar) // Indicatif go brrrrrr
        .for_each(|(i, j, px)| {
            //pixel coordinates as scalars from 0.0 <= t <= 1.0
            //add a little randomness for antialiasing
            let mut colour = v!(0);
            for _ in 0..samples {
                let u = (i as f64 + rand::random::<f64>()) / (img_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (img_height - 1) as f64;

                //get the ray from the camera and then colour it
                let ray = camera.get_ray(u, v);
                colour = colour + ray::colour(&objects, &ray, max_depth);
            }
            //save pixel colour to buffer
            *px = (colour / (samples as f64)).to_rgb();
        });
    buffer.save("render.png").expect("Could not save image");
}

fn random_scene() -> Scene {
    let mut objects: Vec<Box<dyn object::Object + Sync>> = vec![];

    let ground = Box::new(Sphere::new(
        v!(0, -1000, 0),
        1000.0,
        Lambertian::new(v!(0.5, 0.5, 0.5)),
    ));
    objects.push(ground);

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let material_choice: f64 = rand::random();
            let center = v!(
                a + 0.9 * rand::random::<f64>(),
                0.2,
                b + 0.9 * rand::random::<f64>()
            );

            if material_choice < 0.8 {
                //diffuse
                let material = Lambertian::new(v!(rand::random::<f64>()));
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else if material_choice < 0.95 {
                //metal
                let colour = v!(rand::random::<f64>() / 2.0 + 0.5);
                let fuzz = rand::random::<f64>() / 2.0;
                let material = Metal::new(colour, fuzz);
                objects.push(Box::new(Sphere::new(center, 0.2, material)));
            } else {
                //glass
                objects.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        v!(0, 1, 0),
        1.0,
        Dielectric::new(1.5),
    )));
    objects.push(Box::new(Sphere::new(
        v!(-4, 1, 0),
        1.0,
        Lambertian::new(v!(0.4, 0.2, 0.1)),
    )));
    objects.push(Box::new(Sphere::new(
        v!(4, 1, 0),
        1.0,
        Metal::new(v!(0.7, 0.6, 0.5), 0.0),
    )));
    objects
}
