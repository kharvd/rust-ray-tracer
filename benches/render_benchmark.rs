use criterion::{black_box, Criterion};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use rust_ray_tracer::render::{ray_color, render_image};
use rust_ray_tracer::scene::{RenderConfig, setup_small_scene};

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
                black_box(&scene.world),
                black_box(scene.render_config.max_depth)
            )
        });
    });
}

pub fn render_image_benchmark(c: &mut Criterion) {
    let scene = setup_small_scene(RenderConfig {
        image_width: 40,
        image_height: 30,
        samples_per_pixel: 10,
        max_depth: 10,
    });

    c.bench_function("render_image", |b| {
        b.iter(|| {
            render_image(black_box(&scene))
        });
    });
}