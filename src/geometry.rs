use std::sync::Arc;

use rand::thread_rng;

use crate::bounding_box::BBox;
use crate::bvh::BVHNode;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

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

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let orig_to_center = ray.orig - self.center;
        let a = ray.dir.length2();
        let half_b = ray.dir.dot(orig_to_center);
        let c = orig_to_center.length2() - self.radius * self.radius;

        solve_quadratic(a, half_b, c, t_min, t_max).map(|t| {
            let point = ray.at(t);
            let normal = (point - self.center) / self.radius;
            HitRecord::create(
                ray,
                point,
                normal,
                t,
                self.material,
            )
        })
    }

    fn bounding_box(&self) -> BBox {
        let abs_radius = self.radius.abs();
        BBox {
            min: self.center - Vec3::new(abs_radius, abs_radius, abs_radius),
            max: self.center + Vec3::new(abs_radius, abs_radius, abs_radius),
        }
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

pub struct Plane {
    pub center: Point3,
    pub normal: Vec3,
    pub material: Material,
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
                self.material,
            ));
        }

        return None;
    }

    fn bounding_box(&self) -> BBox {
        BBox {
            min: Point3::new(std::f64::NEG_INFINITY, std::f64::NEG_INFINITY, std::f64::NEG_INFINITY),
            max: Point3::new(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vertices: [Point3; 3],
    pub material: Material,
}

impl Hittable for Triangle {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let vertices = &self.vertices;
        let edge1 = vertices[1] - vertices[0];
        let edge2 = vertices[2] - vertices[0];
        let h = ray.dir.cross(edge2);
        let a = edge1.dot(h);
        if a.abs() < 1e-8 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.orig - vertices[0];
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.dir.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t < t_min || t > t_max {
            return None;
        }

        let outward_normal = edge1.cross(edge2).normalize();
        let point = ray.at(t);
        Some(HitRecord::create(
            ray,
            point,
            outward_normal,
            t,
            self.material,
        ))
    }

    fn bounding_box(&self) -> BBox {
        let vertices = &self.vertices;

        let min_x = vertices[0][0].min(vertices[1][0]).min(vertices[2][0]);
        let min_y = vertices[0][1].min(vertices[1][1]).min(vertices[2][1]);
        let min_z = vertices[0][2].min(vertices[1][2]).min(vertices[2][2]);

        let max_x = vertices[0][0].max(vertices[1][0]).max(vertices[2][0]);
        let max_y = vertices[0][1].max(vertices[1][1]).max(vertices[2][1]);
        let max_z = vertices[0][2].max(vertices[1][2]).max(vertices[2][2]);

        BBox {
            min: Point3::new(min_x, min_y, min_z),
            max: Point3::new(max_x, max_y, max_z),
        }
    }
}

pub struct TriangleMesh {
    triangles: BVHNode<Triangle>,
}

impl TriangleMesh {
    pub fn new(vertices: &Vec<Point3>, vertices_index: &Vec<usize>, material: Material) -> TriangleMesh {
        let mut triangles: Vec<Triangle> = vec![];
        for i in 0..vertices_index.len() / 3 {
            triangles.push(Triangle {
                vertices: [
                    vertices[vertices_index[3 * i]],
                    vertices[vertices_index[3 * i + 1]],
                    vertices[vertices_index[3 * i + 2]]
                ],
                material,
            });
        }

        let bvh = BVHNode::from_shapes(&mut thread_rng(), triangles.as_mut_slice());
        TriangleMesh { triangles: bvh }
    }
}

impl Hittable for TriangleMesh {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.triangles.hit_by(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> BBox {
        self.triangles.bounding_box()
    }
}


pub struct Parallelepiped {
    triangles: Vec<Triangle>,
}

impl Parallelepiped {
    pub fn new(a: Point3, b: Point3, c: Point3, d: Point3, material: Material) -> Parallelepiped {
        let v1 = a + ((d - a) + (c - a));
        let v2 = a + ((c - a) + (b - a));
        let v3 = a + ((d - a) + (b - a));
        let v4 = b + ((v3 - b) + (v2 - b));

        Parallelepiped {
            triangles: vec![
                Triangle {
                    vertices: [a, b, c],
                    material,
                },
                Triangle {
                    vertices: [a, d, b],
                    material,
                },
                Triangle {
                    vertices: [a, c, d],
                    material,
                },
                Triangle {
                    vertices: [d, c, v1],
                    material,
                },
                Triangle {
                    vertices: [c, b, v2],
                    material,
                },
                Triangle {
                    vertices: [b, d, v3],
                    material,
                },
                Triangle {
                    vertices: [d, v1, v4],
                    material,
                },
                Triangle {
                    vertices: [d, v4, v3],
                    material,
                },
                Triangle {
                    vertices: [c, v4, v1],
                    material,
                },
                Triangle {
                    vertices: [c, v2, v4],
                    material,
                },
                Triangle {
                    vertices: [b, v4, v2],
                    material,
                },
                Triangle {
                    vertices: [b, v3, v4],
                    material,
                },
            ]
        }
    }
}

impl Hittable for Parallelepiped {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.triangles.hit_by(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> BBox {
        self.triangles.bounding_box()
    }
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

impl<T: Hittable + ?Sized> Hittable for Arc<T> {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        T::hit_by(self, ray, t_min, t_max)
    }

    fn bounding_box(&self) -> BBox {
        T::bounding_box(self)
    }
}

impl<T: Hittable> Hittable for &T {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        T::hit_by(self, ray, t_min, t_max)
    }

    fn bounding_box(&self) -> BBox {
        T::bounding_box(self)
    }
}

pub type ArcHittable = Arc<dyn Hittable + Send + Sync>;