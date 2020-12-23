use crate::ray::Ray;
use crate::geometry::HitRecord;
use crate::vec3::{Color, Vec3};

pub struct ScatteringRecord {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord> {
        let mut target = hit_record.normal + Vec3::random_unit_vector();

        if target.near_zero() {
            target = hit_record.normal;
        }

        let scattered_ray = Ray { orig: hit_record.point, dir: target };
        return Some(ScatteringRecord {
            ray: scattered_ray,
            attenuation: self.albedo,
        });
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord> {
        let dir = ray_in.dir.reflect(&hit_record.normal) + self.fuzz * Vec3::random_in_unit_sphere();
        let scattered_ray = Ray { orig: hit_record.point, dir };
        return Some(ScatteringRecord {
            ray: scattered_ray,
            attenuation: self.albedo,
        });
    }
}