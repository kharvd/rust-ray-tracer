use crate::ray::Ray;

pub trait Hittable {
    fn hit_by(&self, ray: &Ray) -> f64;
}

pub mod sphere {
    use crate::vec3::{Point, Vec3};
    use crate::ray::Ray;
    use crate::geometry::Hittable;

    pub struct Sphere {
        pub center: Point,
        pub radius: f64,
    }

    impl Hittable for Sphere {
        fn hit_by(&self, ray: &Ray) -> f64 {
            let orig_to_center = ray.orig - self.center;
            let a = ray.dir.length2();
            let half_b = ray.dir.dot(&orig_to_center);
            let c = orig_to_center.length2() - self.radius * self.radius;
            let discr = half_b * half_b - a * c;

            if discr < 0.0 {
                return -1.0;
            }

            return (-half_b - discr.sqrt()) / a;
        }
    }

    impl Sphere {
        pub fn normal_at(&self, point: &Point) -> Vec3 {
            return (*point - self.center).normalize();
        }
    }
}
