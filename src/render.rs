use std::f64;

use image::RgbImage;
use itertools::iproduct;
use rand::{Rng, RngCore, SeedableRng, thread_rng};
use rand::rngs::SmallRng;
use rayon::prelude::*;

use crate::color::{Color, put_color};
use crate::geometry::{HitRecord, Shape, Hittable};
use crate::ray::Ray;
use crate::scene::Scene;

pub fn ray_color(rng: &mut dyn RngCore, ray: &Ray, world: &Vec<Shape>, depth: u32) -> Color {
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

fn render_pixel(rng: &mut dyn RngCore, x: u32, y: u32, scene: &Scene) -> Color {
    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;

    let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
    let v = (y as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
    let r = scene.camera.get_ray(rng, u, 1.0 - v);
    ray_color(rng, &r, &scene.world, scene.render_config.max_depth)
}

pub fn render_image_sequential(scene: &Scene) -> RgbImage {
    let mut rng = SmallRng::from_entropy();

    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;
    let samples_per_pixel = scene.render_config.samples_per_pixel;

    let mut img = RgbImage::new(image_width, image_height);
    iproduct!(0..image_width, 0..image_height)
        .map(|(x, y)| {
            let sum = (0..samples_per_pixel)
                .map(|_| render_pixel(&mut rng, x, y, scene))
                .fold(Color::new(0.0, 0.0, 0.0), |accum, x| accum + x);
            (x, y, sum)
        })
        .for_each(|(x, y, color)| {
            put_color(&mut img, x, y, color, scene.render_config.samples_per_pixel);
        });

    img
}

pub fn render_pixel_par(x: u32, y: u32, scene: &Scene, samples_per_pixel: u32) -> Color {
    (0..samples_per_pixel)
        .into_par_iter()
        .map_init(|| SmallRng::from_rng(thread_rng()).unwrap(), |rng, _| render_pixel(rng, x, y, scene))
        .reduce(|| Color::new(0.0, 0.0, 0.0), |accum, x| accum + x)
}

pub fn render_image_parallel(scene: &Scene) -> RgbImage {
    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;
    let samples_per_pixel = scene.render_config.samples_per_pixel;

    let mut result: Vec<(u32, u32, Color)> = Vec::with_capacity((image_width * image_height) as usize);
    iproduct!(0..image_width, 0..image_height)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(x, y)| {
            (*x, *y, render_pixel_par(*x, *y, scene, samples_per_pixel))
        })
        .collect_into_vec(&mut result);

    let mut img = RgbImage::new(image_width, image_height);
    result.iter().for_each(|(x, y, color)| {
        put_color(&mut img, *x, *y, *color, scene.render_config.samples_per_pixel);
    });

    img
}

pub fn render_scene(scene: &Scene, filename: &str, parallel: bool) {
    let img = if parallel {
        render_image_parallel(scene)
    } else {
        render_image_sequential(scene)
    };

    img.save(filename).unwrap();
}
