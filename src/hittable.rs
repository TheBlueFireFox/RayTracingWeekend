use std::{cell::RefCell, rc::Rc};

use crate::{
    cvec::dot,
    ray::{Point, Ray, Vec3},
};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        }
    }
}

pub trait Hittable {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub type HittableObject = Rc<RefCell<dyn Hittable>>;

pub struct HittableList {
    objects: Vec<HittableObject>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Get a reference to the hittable list's objects.
    pub fn objects(&self) -> &[HittableObject] {
        &self.objects
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if obj
                .borrow_mut()
                .hit(r, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
