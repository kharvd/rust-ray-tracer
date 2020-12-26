use criterion::{black_box, Criterion, BenchmarkId};
use rust_ray_tracer::geometry::{Hittable, hit_by};
use rust_ray_tracer::material::Material;
use rust_ray_tracer::point3::Point3;
use rust_ray_tracer::vec3::Vec3;
use rust_ray_tracer::ray::Ray;
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};
use rust_ray_tracer::geometry::Hittable::{SPHERE, PLANE};
use rust_ray_tracer::color::Color;
use rust_ray_tracer::camera::Camera;
use rust_ray_tracer::data::setup_small_scene;

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
                    black_box(std::f64::INFINITY),
                )
            }),
        );
    }
}

pub fn hit_by_list_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42213);
    let mut group = c.benchmark_group("hit_by_list");

    let (shapes, camera) = setup_small_scene();

    group.bench_with_input(
        BenchmarkId::from_parameter("Imperative"),
        &shapes,
        |b, list| b.iter(|| {
            let s = rng.gen();
            let t = rng.gen();
            let ray = camera.get_ray(&mut rng, s, t);

            hit_by(
                list,
                black_box(&ray),
                black_box(std::f64::NEG_INFINITY),
                black_box(std::f64::INFINITY),
            )
        }),
    );

    group.bench_with_input(
        BenchmarkId::from_parameter("Functional"),
        &shapes,
        |b, list| b.iter(|| {
            let s = rng.gen();
            let t = rng.gen();
            let ray = camera.get_ray(&mut rng, s, t);

            hit_by(
                list,
                black_box(&ray),
                black_box(std::f64::NEG_INFINITY),
                black_box(std::f64::INFINITY),
            )
        }),
    );

    group.finish();
}