use criterion::{black_box, Criterion, BenchmarkId};
use rust_ray_tracer::geometry::Hittable;
use rust_ray_tracer::material::Material;
use rust_ray_tracer::point3::Point3;
use rust_ray_tracer::vec3::Vec3;
use rust_ray_tracer::ray::Ray;

pub fn hit_by_benchmark(c: &mut Criterion) {
    let shapes = [
        ("sphere", Hittable::SPHERE {
            center: Point3(0.0, 0.0, 0.0),
            radius: 2.0,
            material: Material::BlackBody,
        }),
        ("plane", Hittable::PLANE {
            center: Point3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 1.0),
            material: Material::BlackBody,
        })
    ];
    let ray = Ray {
        orig: Point3(3.0, 3.0, 3.0),
        dir: Vec3(0.0, 0.0, 0.0),
    };

    let mut group = c.benchmark_group("hit_by");
    for (shape_name, shape) in shapes.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(shape_name),
            &shape,
            |b, hittable| b.iter(|| {
                hittable.hit_by(
                    &ray,
                    black_box(std::f64::NEG_INFINITY),
                    black_box(std::f64::INFINITY)
                )
            }),
        );
    }
}
