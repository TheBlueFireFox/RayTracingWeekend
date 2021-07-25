use std::{rc::Rc};

use crate::{
    cvec::dot,
    material::Material,
    ray::{Point, Ray, Vec3},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub type HittableObject = Rc<dyn Hittable>;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<HittableObject>
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if obj
                .hit(r, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
