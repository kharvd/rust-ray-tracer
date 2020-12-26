use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

mod geometry_benchmark;
mod material_benchmark;
mod render_benchmark;

use crate::geometry_benchmark::{hit_by_benchmark, hit_by_list_benchmark};
use crate::material_benchmark::scatter_benchmark;
use crate::render_benchmark::ray_color_benchmark;

criterion_group!(
    benches,
    hit_by_benchmark,
    hit_by_list_benchmark,
    scatter_benchmark,
    ray_color_benchmark
);
criterion_main!(benches);