use crate::scene::{Scene, random_large_scene};
use crate::color::{Color, print_color};
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng, RngCore};
use crate::ray::Ray;
use crate::geometry::{Hittable, HitRecord};
use std::f64;

fn ray_color(rng: &mut dyn RngCore, ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit_record: Option<HitRecord> = world.hit_by(ray, 0.001, f64::INFINITY);
    return match hit_record {
        Some(rec) => {
            match rec.material.scatter(rng, ray, &rec) {
                Some(scatter_rec) => {
                    scatter_rec.attenuation * ray_color(rng, &scatter_rec.ray, world, depth - 1)
                }

                None => Color::new(0.0, 0.0, 0.0)
            }
        }

        _ => {
            let normalized_dir = ray.dir.normalize();
            let t = 0.5 * (normalized_dir.1 + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    };
}

pub fn render_scene(scene: &Scene) {
    let mut rng = SmallRng::from_entropy();

    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pix = Color::new(0.0, 0.0, 0.0);
            for _s in 0..scene.render_config.samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = scene.camera.get_ray(&mut rng, u, v);
                pix += ray_color(&mut rng, &r, &scene.world, scene.render_config.max_depth);
            }

            print_color(pix, scene.render_config.samples_per_pixel);
        }
    }
}


extern crate test;

use test::Bencher;
use crate::point3::Point3;
use rand::prelude::StdRng;

#[bench]
fn bench_ray_color(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(42123);
    let scene = random_large_scene(&mut rng);

    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;
    let i = 200;
    let j = 300;

    let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
    let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
    let r = scene.camera.get_ray(&mut rng, u, v);

    b.iter(|| {
        ray_color(&mut rng, &r, &scene.world, 10)
    });
}