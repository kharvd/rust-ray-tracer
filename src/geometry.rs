use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use crate::material::Material;
use std::rc::Rc;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn create(
        ray: &Ray,
        point: Point,
        outward_normal: &Vec3,
        t: f64,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        let front_face = ray.dir.dot(outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { -(*outward_normal) };
        return HitRecord {
            point,
            t,
            normal,
            front_face,
            material,
        };
    }
}

pub trait Hittable {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        return self.iter()
            .map(|object| object.hit_by(ray, t_min, t_max))
            .filter_map(|opt_rec| opt_rec)
            .min_by(|rec1, rec2| rec1.t.partial_cmp(&rec2.t).unwrap());
    }
}

pub mod sphere {
    use crate::vec3::{Point, Vec3};
    use crate::ray::Ray;
    use crate::geometry::{Hittable, HitRecord};
    use crate::material::Material;
    use std::rc::Rc;

    pub struct Sphere {
        pub center: Point,
        pub radius: f64,
        pub material: Rc<dyn Material>,
    }

    impl Hittable for Sphere {
        fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
            let orig_to_center = ray.orig - self.center;
            let a = ray.dir.length2();
            let half_b = ray.dir.dot(&orig_to_center);
            let c = orig_to_center.length2() - self.radius * self.radius;
            let discr = half_b * half_b - a * c;

            if discr < 0.0 {
                return Option::None;
            }

            let sqrt_discr = discr.sqrt();

            let t1 = (-half_b - sqrt_discr) / a;
            let t = if t_min < t1 && t1 < t_max {
                t1
            } else {
                let t2 = (-half_b + sqrt_discr) / a;
                if t_min < t2 && t2 < t_max {
                    t2
                } else {
                    return Option::None;
                }
            };

            let point = ray.at(t);
            let normal = self.normal_at(&point);
            return Some(HitRecord::create(
                ray,
                point,
                &normal,
                t,
                self.material.clone(),
            ));
        }
    }

    impl Sphere {
        pub fn normal_at(&self, point: &Point) -> Vec3 {
            return (*point - self.center) / self.radius;
        }
    }
}
