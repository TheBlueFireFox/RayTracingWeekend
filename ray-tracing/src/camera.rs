use crate::{degrees_to_radians, ray::{Point, Ray, Vec3}};

pub const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {

        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
