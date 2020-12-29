use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::bounding_box::BBox;
use Shape::{Sphere, Plane};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn create(
        ray: &Ray,
        point: Point3,
        outward_normal: Vec3,
        t: f64,
        material: Material,
    ) -> HitRecord {
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
    fn bounding_box(&self) -> BBox;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    Sphere {
        center: Point3,
        radius: f64,
        material: Material,
    },
    Plane {
        center: Point3,
        normal: Vec3,
        material: Material,
    },
}

impl Hittable for Shape {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match *self {
            Sphere { material, center, radius } =>
                Shape::sphere_hit_by(ray, t_min, t_max, center, radius, material),
            Plane { center, material, normal } =>
                Shape::plane_hit_by(ray, t_min, t_max, center, normal, material)
        }
    }

    fn bounding_box(&self) -> BBox {
        match *self {
            Sphere { center, radius, .. } => {
                let abs_radius = radius.abs();
                BBox {
                    min: center - Vec3::new(abs_radius, abs_radius, abs_radius),
                    max: center + Vec3::new(abs_radius, abs_radius, abs_radius),
                }
            }
            Plane { .. } => BBox {
                min: Point3::new(std::f64::NEG_INFINITY, std::f64::NEG_INFINITY, std::f64::NEG_INFINITY),
                max: Point3::new(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY),
            }
        }
    }
}

impl Shape {
    fn sphere_hit_by(
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        center: Point3,
        radius: f64,
        material: Material,
    ) -> Option<HitRecord> {
        let orig_to_center = ray.orig - center;
        let a = ray.dir.length2();
        let half_b = ray.dir.dot(orig_to_center);
        let c = orig_to_center.length2() - radius * radius;

        solve_quadratic(a, half_b, c, t_min, t_max).map(|t| {
            let point = ray.at(t);
            let normal = (point - center) / radius;
            HitRecord::create(
                ray,
                point,
                normal,
                t,
                material,
            )
        })
    }

    fn plane_hit_by(
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        center: Point3,
        normal: Vec3,
        material: Material,
    ) -> Option<HitRecord> {
        let dir_dot_normal = ray.dir.dot(normal);
        if dir_dot_normal.abs() < 1e-8 {
            return None;
        }

        let orig_to_center = center - ray.orig;
        let t = orig_to_center.dot(normal) / dir_dot_normal;
        if t_min < t && t < t_max {
            return Some(HitRecord::create(
                ray,
                ray.at(t),
                normal,
                t,
                material,
            ));
        }

        return None;
    }
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

impl<T: Hittable> Hittable for Vec<T> {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_found: Option<HitRecord> = None;

        self.iter()
            .map(|obj| obj.hit_by(ray, t_min, t_max))
            .filter_map(|obj| obj)
            .for_each(|rec| {
                let curr_t = rec.t;
                if closest_t > curr_t {
                    closest_found.replace(rec);
                    closest_t = curr_t;
                }
            });

        closest_found
    }

    fn bounding_box(&self) -> BBox {
        let mut iter = self.iter();
        let first_bbox = match iter.next() {
            None => panic!("No bounding box for an empty set"),
            Some(shape) => shape.bounding_box()
        };

        iter
            .map(|h| h.bounding_box())
            .fold(
                first_bbox,
                |accum, bbox| BBox::surrounding_box(accum, bbox),
            )
    }
}