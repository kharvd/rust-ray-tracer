use criterion::{black_box, Criterion, BenchmarkId};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rust_ray_tracer::color::Color;
use rust_ray_tracer::geometry::HitRecord;
use rust_ray_tracer::material::Material;
use rust_ray_tracer::point3::Point3;
use rust_ray_tracer::vec3::Vec3;
use rust_ray_tracer::ray::Ray;

pub fn scatter_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42213);

    let albedo = Color::random(&mut rng);
    let materials = [
        ("lambertian", Material::LAMBERTIAN { albedo }),
        ("metal", Material::METAL { albedo, fuzz: 0.5 }),
        ("dielectric", Material::DIELECTRIC { index_of_refraction: 1.5 }),
        ("black_body", Material::BlackBody)
    ];
    let angle = 45.0;
    let angle_rad = angle * std::f64::consts::PI / 180.0;
    let ray = Ray {
        orig: Point3(0.0, 0.0, 0.0),
        dir: Vec3(1.0, 0.0, 0.0),
    };

    let mut group = c.benchmark_group("scatter");
    for (mat_name, material) in materials.iter() {
        let hit_record = HitRecord {
            material: *material,
            t: 1.0,
            front_face: true,
            point: Point3(1.0, 0.0, 0.0),
            normal: Vec3(angle_rad.cos(), angle_rad.sin(), 0.0),
        };
        group.bench_with_input(
            BenchmarkId::from_parameter(mat_name),
            &hit_record,
            |b, rec| b.iter(|| {
                material.scatter(&mut rng, black_box(&ray), rec)
            }),
        );
    }
}
