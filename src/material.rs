use crate::ray::Ray;
use crate::geometry::HitRecord;
use crate::vec3::Vec3;
use rand::{RngCore, Rng};
use crate::color::Color;
use serde::{Serialize, Deserialize};

pub struct ScatteringRecord {
    pub ray: Ray,
    pub attenuation: Color,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Material {
    Lambertian { albedo: Color },
    Metal {
        albedo: Color,
        fuzz: f64,
    },
    Dielectric {
        index_of_refraction: f64,
    },
    BlackBody,
}

impl Material {
    pub fn scatter(&self,
                   rng: &mut dyn RngCore,
                   ray_in: &Ray,
                   hit_record: &HitRecord
    ) -> Option<ScatteringRecord> {
        match *self {
            Material::Lambertian { albedo } =>
                Material::scatter_lambertian(rng, hit_record, albedo),
            Material::Metal { albedo, fuzz } =>
                Material::scatter_metal(rng, ray_in, hit_record, albedo, fuzz),
            Material::Dielectric { index_of_refraction } =>
                Material::scatter_dielectric(rng, ray_in, hit_record, index_of_refraction),
            Material::BlackBody => None
        }
    }

    fn scatter_lambertian(
        rng: &mut dyn RngCore,
        hit_record: &HitRecord,
        albedo: Color
    ) -> Option<ScatteringRecord> {
        let mut target = hit_record.normal + Vec3::random_unit_vector(rng);

        if target.near_zero() {
            target = hit_record.normal;
        }

        let scattered_ray = Ray { orig: hit_record.point, dir: target };
        return Some(ScatteringRecord {
            ray: scattered_ray,
            attenuation: albedo,
        });
    }

    fn scatter_metal(
        rng: &mut dyn RngCore,
        ray_in: &Ray,
        hit_record: &HitRecord,
        albedo: Color,
        fuzz: f64
    ) -> Option<ScatteringRecord> {
        let dir = ray_in.dir.reflect(hit_record.normal) + fuzz * Vec3::random_in_unit_sphere(rng);
        let scattered_ray = Ray { orig: hit_record.point, dir };
        return Some(ScatteringRecord {
            ray: scattered_ray,
            attenuation: albedo,
        });
    }

    fn scatter_dielectric(
        rng: &mut dyn RngCore,
        ray_in: &Ray,
        hit_record: &HitRecord,
        index_of_refraction: f64
    ) -> Option<ScatteringRecord> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / index_of_refraction
        } else {
            index_of_refraction
        };

        let unit_direction = ray_in.dir.normalize();
        let cos_theta = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let should_reflect = cannot_refract ||
            reflectance(cos_theta, refraction_ratio) > rng.gen();

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

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
