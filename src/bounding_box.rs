use crate::point3::Point3;
use crate::ray::Ray;
use std::mem::swap;

#[derive(Debug, Clone, Copy)]
pub struct BBox {
    pub min: Point3,
    pub max: Point3,
}

impl BBox {
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir[i];
            let mut t0 = (self.min[i] - ray.orig[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.orig[i]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding_box(box1: BBox, box2: BBox) -> BBox {
        let min = Point3::new(
            box1.min[0].min(box2.min[0]),
            box1.min[1].min(box2.min[1]),
            box1.min[2].min(box2.min[2])
        );
        let max = Point3::new(
            box1.max[0].max(box2.max[0]),
            box1.max[1].max(box2.max[1]),
            box1.max[2].max(box2.max[2])
        );
        BBox { min, max }
    }
}
