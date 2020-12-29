use criterion::{criterion_group, criterion_main};

use crate::geometry_benchmark::{hit_by_benchmark, hit_by_list_benchmark, bvh_benchmark};
use crate::material_benchmark::scatter_benchmark;
use crate::render_benchmark::{ray_color_benchmark, render_image_benchmark};

mod geometry_benchmark;
mod material_benchmark;
mod render_benchmark;

criterion_group!(
    benches,
    hit_by_benchmark,
    hit_by_list_benchmark,
    bvh_benchmark,
    scatter_benchmark,
    ray_color_benchmark,
    render_image_benchmark
);
criterion_main!(benches);