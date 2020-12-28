use crate::point3::Point3;
use crate::ray::Ray;

pub struct BBox {
    pub min: Point3,
    pub max: Point3,
}

impl BBox {
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let min = self.min.as_slice();
        let max = self.max.as_slice();
        let orig = ray.orig.as_slice();
        let dir = ray.dir.as_slice();

        for i in 0..3 {
            let t0 = (min[i] - orig[i]) / dir[i];
            let t1 = (max[i] - orig[i]) / dir[i];
            let (t0, t1) = (t0.min(t1), t0.max(t1));

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_min > t_max {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding_box(box1: BBox, box2: BBox) -> BBox {
        let min = Point3(
            box1.min.0.min(box2.min.0),
            box1.min.1.min(box2.min.1),
            box1.min.2.min(box2.min.2)
        );
        let max = Point3(
            box1.max.0.max(box2.max.0),
            box1.max.1.max(box2.max.1),
            box1.max.2.max(box2.max.2)
        );
        BBox { min, max }
    }
}
