use std::{cell::RefCell, rc::Rc};

use image::Color;
use rand::random;
use ray::{Point, Ray, Vec3};

use crate::{camera::Camera, image::Image, sphere::Sphere};

mod camera;
mod cvec;
mod hittable;
mod image;
mod ppm;
mod ray;
mod rtweekend;
mod sphere;

fn ray_color<H: hittable::Hittable>(r: &Ray, world: &mut H) -> Color {
    let mut rec = Default::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let path = "main";

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    let mut world = hittable::HittableList::new();
    let mut adder = |v| world.add(Rc::new(RefCell::new(v)));
    adder(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    adder(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let cam = Camera::new();

    // Render
    let mut data = Vec::with_capacity(image_height * image_width);

    println!("Running");

    let calc = |o, l| ((o as f64) + rand::random::<f64>()) / (l - 1) as f64;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {

            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..samples_per_pixel {
                let u = calc(i, image_width);
                let v = calc(j, image_height);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &mut world);
            }

            data.push(pixel_color);
        }
    }

    let img = Image::new(&data, image_height, image_width,samples_per_pixel);

    ppm::save(img, path).expect("Something went terribly wrong here");

    println!("\nDone");
}
