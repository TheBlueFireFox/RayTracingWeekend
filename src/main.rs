use cvec::unit_vector;
use image::Color;
use ray::Ray;

mod ppm;
mod image;
mod cvec;
mod ray;

fn ray_color(r : &Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0 );
    (1.0 - t) * Color::new(0.0,0.0,0.0) + t * Color::new(0.5,0.7,1.0)
}

fn main() {
}
