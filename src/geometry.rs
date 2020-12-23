use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit_by(&self, ray: &Ray) -> Option<HitRecord>;
}

pub mod sphere {
    use crate::vec3::{Point, Vec3};
    use crate::ray::Ray;
    use crate::geometry::{Hittable, HitRecord};

    pub struct Sphere {
        pub center: Point,
        pub radius: f64,
    }

    impl Hittable for Sphere {
        fn hit_by(&self, ray: &Ray) -> Option<HitRecord> {
            let orig_to_center = ray.orig - self.center;
            let a = ray.dir.length2();
            let half_b = ray.dir.dot(&orig_to_center);
            let c = orig_to_center.length2() - self.radius * self.radius;
            let discr = half_b * half_b - a * c;

            if discr < 0.0 {
                return Option::None;
            }

            let t = (-half_b - discr.sqrt()) / a;
            let point = ray.at(t);
            let normal = self.normal_at(&point);
            return Some(HitRecord { point, normal, t });
        }
    }

    impl Sphere {
        pub fn normal_at(&self, point: &Point) -> Vec3 {
            return (*point - self.center).normalize();
        }
    }
}
