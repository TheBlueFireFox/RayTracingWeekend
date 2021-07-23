use progressing::{
    mapping::Bar as MappingBar,
    // The underlying Trait
    Baring,
};
use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use ray_tracing::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    ray::{Point, Ray, Vec3},
    render::ppm,
    render::Color,
    render::Image,
    sphere::Sphere,
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Diffuse {
    Lambertian,
    UniformScatter
}


fn ray_color<H: Hittable>(r: &Ray, world: &mut H, depth: usize, dif: Diffuse) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = Default::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = match dif {
            Diffuse::Lambertian => rec.p + rec.normal + Vec3::random_unit_vector(),
            Diffuse::UniformScatter => rec.p + rec.normal.random_in_hemisphere(),
        };
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1, dif);
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
    let max_depth = 50;
    let gamma = 2.0;

    let mut world = HittableList::new();
    let mut adder = |v| world.add(Rc::new(RefCell::new(v)));
    adder(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    adder(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    // Diffuser
    let diff = Diffuse::Lambertian;

    // progress bar
    let mut bar = MappingBar::with_range(0, image_height).timed();
    bar.set_len(20);
    bar.set(0usize);

    // Camera
    let cam = Camera::new();

    // Render
    let mut data = Vec::with_capacity(image_height * image_width);

    println!("Running");

    let calc = |o, l| ((o as f64) + ray_tracing::random::<f64>()) / (l - 1) as f64;

    for j in (0..image_height).rev() {
        bar.set(image_height - j);
        print!("\r{} ", bar);
        let _ = io::stdout().flush();
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let v = calc(j, image_height);
                let u = calc(i, image_width);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &mut world, max_depth, diff);
            }

            data.push(pixel_color);
        }
    }

    let img = Image::new(&data, image_height, image_width, samples_per_pixel, gamma);

    ppm::save(img, path).expect("Something went terribly wrong here");

    println!("\nDone");
}
