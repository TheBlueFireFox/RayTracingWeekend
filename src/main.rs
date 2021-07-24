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
    material::{Dielectric, Lambartian, Material, Metal},
    rand_range,
    ray::{Point, Ray, Vec3},
    render::ppm,
    render::Color,
    render::Image,
    sphere::Sphere,
};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut adder_o = |(x, y, z), r, m| {
        let sphere = Sphere::new(Point::new(x, y, z), r, m);
        world.add(Rc::new(RefCell::new(sphere)));
    };

    let mut adder = |p: Point, r, m| {
        let data = p.data();
        adder_o((data[0], data[1], data[2]), r, m)
    };

    let make_lam_o = |(x, y, z)| Rc::new(RefCell::new(Lambartian::new(Color::new(x, y, z))));
    let make_met_o = |(x, y, z), f| Rc::new(RefCell::new(Metal::new(Color::new(x, y, z), f)));
    let make_diel_o = |x| Rc::new(RefCell::new(Dielectric::new(x)));

    let ground_material = make_lam_o((0.5, 0.5, 0.5));
    adder(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material);

    let make_lam = |p: Color| {
        let data = p.data();
        make_lam_o((data[0], data[1], data[1]))
    };

    let make_met = |p: Color, f| {
        let data = p.data();
        make_met_o((data[0], data[1], data[1]), f)
    };

    let make_diel = |v| make_diel_o(v);

    let calc = |v| (v as f64) + 0.9 * rand_range(0.0..1.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_range(0.0..1.0);
            let center = Point::new(calc(a), 0.2, calc(b));

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<RefCell<dyn Material>>;

                sphere_material = if choose_mat < 0.5 {
                    let albedo = Color::random_range(0.2..1.0) * Color::random_range(0.2..1.0);
                    make_lam(albedo)
                } else if choose_mat < 0.85 {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand_range(0.0..0.5);
                    make_met(albedo, fuzz)
                } else {
                    make_diel(1.5)
                };

                adder(center, 0.2, sphere_material);
            }
        }
    }
    let a: &[((f64, f64, f64), Rc<RefCell<dyn Material>>)] = &[
        ((0.0, 1.0, 0.0), make_diel(1.5)),
        ((-4.0, 1.0, 0.0), make_lam_o((0.4, 0.2, 0.1))),
        ((4.0, 1.0, 0.0), make_met_o((0.7, 0.6, 0.5), 0.0)),
    ];

    for m in a {
        let p = Point::new(m.0 .0, m.0 .1, m.0 .2);
        adder(p, 1.0, m.1.clone());
    }

    world
}

fn ray_color<H: Hittable>(r: &Ray, world: &mut H, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = Default::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);

        if let Some(ref mat) = rec.mat {
            if mat
                .borrow()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }

            return Color::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let path = "main";

    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let gamma = 2.0;

    // World

    let mut world = random_scene();

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // progress bar
    let mut bar = MappingBar::with_range(0, image_height).timed();
    bar.set_len(20);
    bar.set(0usize);

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
                pixel_color += ray_color(&r, &mut world, max_depth);
            }

            data.push(pixel_color);
        }
    }

    println!();
    println!("Writing to file");

    let img = Image::new(&data, image_height, image_width, samples_per_pixel, gamma);

    ppm::save(img, path).expect("Something went terribly wrong here");

    println!("Done");
}
