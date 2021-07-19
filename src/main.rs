use cvec::Point;
use image::Color;
use ray::{Ray, Vec3};

use crate::image::Image;

mod cvec;
mod image;
mod ppm;
mod ray;

fn ray_color(r: Ray) -> Color {
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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_lenght);

    // Render
    let mut data = Vec::with_capacity(image_height * image_width);

    println!("Running");

    let calc = |o, l| (o as f64) / (l - 1) as f64;

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let v = calc(j, image_height);
            let u = calc(i, image_width);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(r);

            data.push(pixel_color);
        }
    }

    let img = Image::new(&data, image_height, image_width);

    ppm::save(img, path).expect("Something went terribly wrong here");

    println!("\nDone");
}
