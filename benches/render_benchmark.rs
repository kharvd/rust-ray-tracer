use criterion::{BenchmarkId, black_box, Criterion};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use rust_ray_tracer::data::setup_small_scene;
use rust_ray_tracer::render::ray_color;

pub fn ray_color_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42213);
    let (shapes, camera) = setup_small_scene();

    c.bench_function("ray_color", |b| {
        let s = rng.gen();
        let t = rng.gen();
        let ray = camera.get_ray(&mut rng, s, t);

        b.iter(|| {
            ray_color(&mut rng, black_box(&ray), black_box(&shapes), black_box(10))
        });
    });
}