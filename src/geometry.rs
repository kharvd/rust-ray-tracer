use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::point3::Point3;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn create(
        ray: &Ray,
        point: Point3,
        outward_normal: Vec3,
        t: f64,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        let front_face = ray.dir.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
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

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_found: Option<HitRecord> = None;
        for object in self {
            if let Some(rec) = object.hit_by(ray, t_min, t_max) {
                let curr_t = rec.t;
                if closest_t > curr_t {
                    closest_found.replace(rec);
                    closest_t = curr_t;
                }
            }
        }

        return closest_found;
    }
}

pub mod sphere {
    use crate::vec3::Vec3;
    use crate::point3::Point3;
    use crate::ray::Ray;
    use crate::geometry::{Hittable, HitRecord};
    use crate::material::{Material, Lambertian};
    use std::borrow::Borrow;

    pub struct Sphere {
        pub center: Point3,
        pub radius: f64,
        pub material: Box<dyn Material>,
    }

    #[inline]
    fn solve_quadratic(a: f64, half_b: f64, c: f64, t_min: f64, t_max: f64) -> Option<f64> {
        let discr = half_b * half_b - a * c;
        if discr < 0.0 {
            return None;
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

        return Some(t);
    }


    impl Hittable for Sphere {
        fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
            let orig_to_center = ray.orig - self.center;
            let a = ray.dir.length2();
            let half_b = ray.dir.dot(orig_to_center);
            let c = orig_to_center.length2() - self.radius * self.radius;

            solve_quadratic(a, half_b, c, t_min, t_max).map(|t| {
                let point = ray.at(t);
                let normal = self.normal_at(point);
                HitRecord::create(
                    ray,
                    point,
                    normal,
                    t,
                    self.material.borrow(),
                )
            })
        }
    }

    impl Sphere {
        pub fn normal_at(&self, point: Point3) -> Vec3 {
            return (point - self.center) / self.radius;
        }
    }

    extern crate test;

    use test::Bencher;
    use rand::rngs::SmallRng;
    use rand::{SeedableRng, Rng};
    use crate::color::Color;

    #[bench]
    fn bench_sphere_hit_by(b: &mut Bencher) {
        let mut rng = SmallRng::from_entropy();
        let sphere = Sphere {
            center: Point3(rng.gen(), rng.gen(), rng.gen()),
            radius: rng.gen::<f64>() * 50.0,
            material: Box::new(Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            }),
        };

        let point3 = Point3(rng.gen(), rng.gen(), rng.gen());
        let vec3 = Vec3::random(&mut rng);
        let ray = Ray {
            orig: point3,
            dir: vec3,
        };

        b.iter(|| {
            sphere.hit_by(&ray, std::f64::NEG_INFINITY, std::f64::INFINITY)
        });
    }
}

pub mod plane {
    use crate::point3::Point3;
    use crate::vec3::Vec3;
    use crate::material::{Material, Lambertian};
    use crate::geometry::{Hittable, HitRecord};
    use crate::ray::Ray;
    use std::borrow::Borrow;

    pub struct Plane {
        pub center: Point3,
        pub normal: Vec3,
        pub material: Box<dyn Material>,
    }

    impl Hittable for Plane {
        fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
            let dir_dot_normal = ray.dir.dot(self.normal);
            if dir_dot_normal.abs() < 1e-8 {
                return None;
            }

            let orig_to_center = self.center - ray.orig;
            let t = orig_to_center.dot(self.normal) / dir_dot_normal;
            if t_min < t && t < t_max {
                return Some(HitRecord::create(
                    ray,
                    ray.at(t),
                    self.normal,
                    t,
                    self.material.borrow(),
                ));
            }

            return None;
        }
    }

    extern crate test;

    use test::Bencher;
    use rand::rngs::SmallRng;
    use rand::{SeedableRng, Rng};
    use crate::color::Color;
    use rand::prelude::StdRng;

    #[bench]
    fn bench_plane_hit_by(b: &mut Bencher) {
        let mut rng = SmallRng::from_entropy();
        let plane = Plane {
            center: Point3(rng.gen(), rng.gen(), rng.gen()),
            normal: Vec3::random(&mut rng),
            material: Box::new(Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            }),
        };

        let point3 = Point3(rng.gen(), rng.gen(), rng.gen());
        let vec3 = Vec3::random(&mut rng);

        let ray = Ray {
            orig: point3,
            dir: vec3,
        };

        b.iter(|| {
            plane.hit_by(&ray, std::f64::NEG_INFINITY, std::f64::INFINITY)
        });
    }
}
