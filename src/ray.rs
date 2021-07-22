pub type Point = crate::cvec::Point<f64>;
pub type Vec3 = crate::cvec::Vec3<f64>;

pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> Point {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
