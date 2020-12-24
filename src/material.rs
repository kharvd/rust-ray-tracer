use crate::ray::Ray;
use crate::geometry::HitRecord;
use crate::vec3::Vec3;
use rand::{random, RngCore};
use crate::color::Color;

pub struct ScatteringRecord {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, rng: &mut dyn RngCore, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut dyn RngCore, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord> {
        let mut target = hit_record.normal + Vec3::random_unit_vector(rng);

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
    fn scatter(&self, rng: &mut dyn RngCore, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord> {
        let dir = ray_in.dir.reflect(hit_record.normal) + self.fuzz * Vec3::random_in_unit_sphere(rng);
        let scattered_ray = Ray { orig: hit_record.point, dir };
        return Some(ScatteringRecord {
            ray: scattered_ray,
            attenuation: self.albedo,
        });
    }
}

pub struct Dielectric {
    pub index_of_refraction: f64,
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

impl Material for Dielectric {
    fn scatter(&self, _rng: &mut dyn RngCore, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteringRecord> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.dir.normalize();
        let cos_theta = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let should_reflect = cannot_refract ||
            reflectance(cos_theta, refraction_ratio) > random::<f64>();

        let dir = if should_reflect {
            unit_direction.reflect(hit_record.normal)
        } else {
            Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let scattered_ray = Ray { orig: hit_record.point, dir };
        return Some(ScatteringRecord {
            ray: scattered_ray,
            attenuation: Color::new(1.0, 1.0, 1.0),
        });
    }
}