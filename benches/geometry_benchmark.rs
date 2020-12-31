use criterion::{BenchmarkId, black_box, Criterion};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use rust_ray_tracer::geometry::{Shape, Hittable};
use rust_ray_tracer::material::Material;
use rust_ray_tracer::point3::Point3;
use rust_ray_tracer::ray::Ray;
use rust_ray_tracer::scene::{RenderConfig, setup_small_scene, setup_scene};
use rust_ray_tracer::vec3::Vec3;
use rust_ray_tracer::bvh::BVHNode;

pub fn hit_by_benchmark(c: &mut Criterion) {
    let shapes = [
        ("sphere", Shape::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            material: Material::BlackBody,
        }),
        ("plane", Shape::Plane {
            center: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: Material::BlackBody,
        })
    ];
    let ray = Ray {
        orig: Point3::new(3.0, 3.0, 3.0),
        dir: Vec3::new(0.0, 0.0, 0.0),
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

    let scene = setup_small_scene(RenderConfig {
        image_width: 40,
        image_height: 30,
        samples_per_pixel: 1,
        max_depth: 10,
    });

    group.bench_with_input(
        BenchmarkId::from_parameter("fast"),
        &scene.hittables,
        |b, list| b.iter(|| {
            let ray = Ray {
                orig: Point3::new(rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)),
                dir: Vec3::random_unit_vector(&mut rng),
            };

            list.hit_by(
                black_box(&ray),
                black_box(std::f64::NEG_INFINITY),
                black_box(std::f64::INFINITY),
            )
        }),
    );

    group.finish();
}

pub fn bvh_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42213);
    let mut group = c.benchmark_group("hit_by_bvh");

    for num_spheres in [10, 100, 500, 1000].iter() {
        let scene = setup_scene(&mut rng, RenderConfig {
            image_width: 40,
            image_height: 30,
            samples_per_pixel: 1,
            max_depth: 1,
        }, *num_spheres);

        group.bench_with_input(
            BenchmarkId::new("hit_by_list", num_spheres),
            &scene.hittables,
            |b, list| b.iter(|| {
                let ray = Ray {
                    orig: Point3::new(rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)),
                    dir: Vec3::random_unit_vector(&mut rng),
                };

                list.hit_by(
                    black_box(&ray),
                    black_box(0.001),
                    black_box(std::f64::INFINITY),
                )
            }),
        );

        let bvh = BVHNode::from_shapes(&mut rng, scene.hittables.clone().as_mut_slice());

        group.bench_with_input(
            BenchmarkId::new("hit_by_bvh", num_spheres),
            &bvh,
            |b, bvh| b.iter(|| {
                let ray = Ray {
                    orig: Point3::new(rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)),
                    dir: Vec3::random_unit_vector(&mut rng),
                };

                bvh.hit_by(
                    black_box(&ray),
                    black_box(0.001),
                    black_box(std::f64::INFINITY),
                )
            }),
        );
    }


    group.finish();
}