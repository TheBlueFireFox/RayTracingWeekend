use crate::{cvec::dot, hittable::Hittable, ray::Point};


pub struct Sphere {
   pub center: Point,
   pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &crate::ray::Ray, t_min: f64, t_max : f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / (self.radius);

        true
    }
}