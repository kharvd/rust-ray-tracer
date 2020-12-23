use crate::vec3::Vec3;
use crate::vec3::Point;

pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point {
        return self.orig + self.dir * t;
    }
}