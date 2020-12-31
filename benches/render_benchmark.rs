use criterion::{black_box, Criterion, BenchmarkId};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use rust_ray_tracer::render::{ray_color, render_image_sequential, render_image_parallel};
use rust_ray_tracer::scene::{RenderConfig, setup_small_scene};
use std::time::Duration;

pub fn ray_color_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42213);
    let scene = setup_small_scene(RenderConfig {
        image_width: 40,
        image_height: 30,
        samples_per_pixel: 1,
        max_depth: 10,
    });

    c.bench_function("ray_color", |b| {
        let s = rng.gen();
        let t = rng.gen();
        let ray = scene.camera.get_ray(&mut rng, s, t);

        b.iter(|| {
            ray_color(
                &mut rng,
                black_box(&ray),
                black_box(&scene.hittables),
                black_box(scene.render_config.max_depth)
            )
        });
    });
}

pub fn render_image_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_image");

    group
        .sample_size(30)
        .measurement_time(Duration::new(10, 0));

    [1, 10, 100].iter().for_each(|samples_per_pixel| {
        let scene = setup_small_scene(RenderConfig {
            image_width: 40,
            image_height: 30,
            samples_per_pixel: *samples_per_pixel,
            max_depth: 10,
        });

        group.bench_with_input(
            BenchmarkId::new("sequential_samples", samples_per_pixel),
            samples_per_pixel,
            |b, _| b.iter(|| {
                render_image_sequential(black_box(&scene), black_box(&scene.hittables))
            }),
        );

        group.bench_with_input(
            BenchmarkId::new("parallel_samples", samples_per_pixel),
            samples_per_pixel,
            |b, _| b.iter(|| {
                render_image_parallel(black_box(&scene), black_box(&scene.hittables))
            }),
        );
    });

    [1, 10, 100].iter().for_each(|size| {
        let scene = setup_small_scene(RenderConfig {
            image_width: 4 * size,
            image_height: 300,
            samples_per_pixel: 1,
            max_depth: 10,
        });

        group.bench_with_input(
            BenchmarkId::new("sequential_size", size),
            size,
            |b, _| b.iter(|| {
                render_image_sequential(black_box(&scene), black_box(&scene.hittables))
            }),
        );

        group.bench_with_input(
            BenchmarkId::new("parallel_size", size),
            size,
            |b, _| b.iter(|| {
                render_image_parallel(black_box(&scene), black_box(&scene.hittables))
            }),
        );
    });
}