use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use std::sync::Arc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use ray_tracing::{
    camera::Camera,
    clamp,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambartian, Mat, Material, Metal},
    rand_range,
    ray::{Point, Ray, Vec3},
    render::Color,
    sphere::Sphere,
};

pub const REPETITION: usize = 4;
pub const ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMAGE_WIDTH: usize = 1200;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
pub const SAMPLES_PER_PIXEL: usize = 500;
pub const MAX_DEPTH: usize = 50;
pub const GAMMA: f64 = 2.0;

fn random_scene() -> HittableList {
    let mut world = HittableList::with_capacity(11 * 2 * 2);

    let mut adder_o = |(x, y, z), r, m| {
        let sphere = Sphere::new(Point::new(x, y, z), r, m);
        world.add(Arc::new(sphere));
    };

    let mut adder = |p: Point, r, m| {
        let data = p.data();
        adder_o((data[0], data[1], data[2]), r, m)
    };

    let make_lam_o = |(x, y, z)| Arc::new(Lambartian::new(Color::new(x, y, z)));
    let make_met_o = |(x, y, z), f| Arc::new(Metal::new(Color::new(x, y, z), f));
    let make_diel_o = |x| Arc::new(Dielectric::new(x));

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
                let sphere_material: Mat;

                sphere_material = if choose_mat < 0.5 {
                    let albedo = Color::random_range(0.05..0.95) * Color::random_range(0.05..0.95);
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
    let a: &[((f64, f64, f64), Arc<dyn Material>)] = &[
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

fn ray_color<H: Hittable>(r: &Ray, world: &H, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = Default::default();

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);

        if let Some(ref mat) = rec.mat {
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }

            return Color::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn irun<H: Hittable>(world: &H, pb: ProgressBar) -> Vec<Color> {
    pb.set_position(0);

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
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    // Render

    let calc = |o, l| ((o as f64) + ray_tracing::rand_range(0.0..1.0)) / (l - 1) as f64;

    // Divide the color by the number of samples
    let fix_scale = 1.0 / (SAMPLES_PER_PIXEL as f64);

    // gamma and clamping the values
    let fix_pixel_val = |v: f64| {
        let v = (fix_scale * v).powf(1.0 / GAMMA);
        let c = clamp(v, 0.0, 0.999);
        256.0 * c
    };

    let fix_pixel = |p: Color| {
        let r = fix_pixel_val(p.x());
        let g = fix_pixel_val(p.y());
        let b = fix_pixel_val(p.z());

        Color::new(r, g, b)
    };

    // let mut data = Vec::with_capacity(image_height * image_width);
    let outer: Vec<_> = (0..IMAGE_HEIGHT).rev().collect();

    let data: Vec<_> = outer
        .par_iter()
        .map(|&j| {
            let mut idata = Vec::with_capacity(IMAGE_WIDTH);

            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..SAMPLES_PER_PIXEL {
                    let v = calc(j, IMAGE_HEIGHT);
                    let u = calc(i, IMAGE_WIDTH);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, world, MAX_DEPTH);
                }

                idata.push(fix_pixel(pixel_color));
            }

            idata
        })
        .progress_with(pb)
        .flatten()
        .collect();
    data
}

pub fn run(pb_run: ProgressBar, pb_int: ProgressBar) -> Vec<Color> {
    pb_run.set_position(0);

    // World
    let world = random_scene();

    // run 
    let mut tmp : Vec<_> = (0..REPETITION)
        .map(|_| Some(irun(&world, pb_int.clone())))
        .progress_with(pb_run)
        .collect();

    // prepare the solution
    let mut res = tmp[0].take().unwrap();

    for arr in tmp.iter().skip(1) {
        let arr = arr.as_ref().unwrap();
        for (i, val) in arr.iter().enumerate() {
            res[i] += *val;
        }
    }

    for val in res.iter_mut() {
        *val /= tmp.len() as f64;
    }

    res
}
