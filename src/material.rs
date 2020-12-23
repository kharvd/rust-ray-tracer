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
        let target = hit_record.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray { orig: hit_record.point, dir: target };
        return Some(ScatteringRecord{
            ray: scattered_ray,
            attenuation: self.albedo
        })
    }
}