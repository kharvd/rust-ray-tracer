use std::f64;

use image::{Rgb, RgbImage, ImageBuffer, ImageFormat};
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::SmallRng;

use crate::color::{Color, print_color, put_color};
use crate::geometry::{hit_by, HitRecord, Hittable};
use crate::ray::Ray;
use crate::scene::Scene;

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

pub fn render_image(scene: &Scene) -> RgbImage {
    let mut rng = SmallRng::from_entropy();

    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;

    let mut img = RgbImage::new(image_width, image_height);

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

            put_color(&mut img, i, image_height - j - 1, pix, scene.render_config.samples_per_pixel);
        }
    }

    img
}

pub fn render_scene(scene: &Scene, filename: &str) {
    let img = render_image(scene);
    img.save(filename).unwrap();
}
