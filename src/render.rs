use std::f64;
use std::sync::mpsc::channel;

use image::{ImageBuffer, Rgb, RgbImage};
use itertools::iproduct;
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::SmallRng;
use rayon::prelude::*;

use crate::color::Color;
use crate::geometry::{hit_by, HitRecord, Hittable};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::color;

pub fn ray_color(rng: &mut dyn RngCore, ray: &Ray, world: &Vec<Hittable>, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit_record: Option<HitRecord> = hit_by(&world, ray, 0.001, f64::INFINITY);
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

fn render_pixel(x: u32, y: u32, scene: &Scene) -> Color {
    let mut rng: SmallRng = SmallRng::from_entropy();

    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;

    let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
    let v = (y as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
    let r = scene.camera.get_ray(&mut rng, u, 1.0 - v);
    ray_color(&mut rng, &r, &scene.world, scene.render_config.max_depth)
}

pub fn render_image_sequential(scene: &Scene) -> RgbImage {
    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;
    let samples_per_pixel = scene.render_config.samples_per_pixel;

    let mut img_buffer: ImageBuffer<Rgb<f64>, _> = ImageBuffer::new(image_width, image_height);
    iproduct!(0..image_width, 0..image_height, 0..samples_per_pixel)
        .map(|(x, y, _)| {
            (x, y, render_pixel(x, y, scene))
        })
        .for_each(|(x, y, color)| {
            color::update_pixel(&mut img_buffer, x, y, color);
        });

    color::discretize_image(&img_buffer, samples_per_pixel)
}

pub fn render_image_parallel(scene: &Scene) -> RgbImage {
    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;
    let samples_per_pixel = scene.render_config.samples_per_pixel;

    let (sender, receiver) = channel::<(u32, u32, Color)>();

    iproduct!(0..image_width, 0..image_height, 0..samples_per_pixel)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(x, y, _)| {
            (*x, *y, render_pixel(*x, *y, scene))
        })
        .for_each_with(sender, |s, tup| {
            s.send(tup).unwrap()
        });

    let mut img_buffer: ImageBuffer<Rgb<f64>, _> = ImageBuffer::new(image_width, image_height);
    receiver.iter().for_each(|(x, y, color)| {
        color::update_pixel(&mut img_buffer, x, y, color);
    });

    color::discretize_image(&mut img_buffer, samples_per_pixel)
}

pub fn render_scene(scene: &Scene, filename: &str, parallel: bool) {
    let img = if (parallel) {
        render_image_parallel(scene)
    } else {
        render_image_sequential(scene)
    };

    img.save(filename).unwrap();
}
