use crate::scene::Scene;
use crate::color::{Color, print_color};
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng, RngCore};
use crate::ray::Ray;
use crate::geometry::{Hittable, HitRecord};
use std::f64;
use crate::lighting::{LightSource, PointLight};
use crate::point3::Point3;

fn ray_color(rng: &mut dyn RngCore, ray: &Ray, light: &dyn LightSource, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit_record: Option<HitRecord> = world.hit_by(ray, 0.001, f64::INFINITY);
    return match hit_record {
        Some(rec) => {
            match rec.material.scatter(rng, ray, &rec) {
                Some(scatter_rec) => {
                    let illum_rec = light.illuminate(rec.point);
                    let shadow_ray_hit = world.hit_by(&illum_rec.ray, 0.001, illum_rec.t);
                    let illum_weight = if shadow_ray_hit.is_some() {
                        0.0
                    } else {
                        illum_rec.intensity
                    };

                    let scattered_color = ray_color(rng, &scatter_rec.ray, light, world, depth - 1);
                    let illum_color = illum_rec.color;

                    scatter_rec.attenuation * ((1.0 - illum_weight) * scattered_color + illum_weight * illum_color)
                }

                None => Color::new(0.0, 0.0, 0.0)
            }
        }

        _ => {
            let normalized_dir = ray.dir.normalize();
            let t = 0.5 * (normalized_dir.1 + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            // Color::new(0.0, 0.0, 0.0)
        }
    };
}

pub fn render_scene(scene: &Scene) {
    let mut rng = SmallRng::from_entropy();

    let image_width = scene.render_config.image_width;
    let image_height = scene.render_config.image_height;

    let light = PointLight {
        color: Color::new(1.0, 1.0, 1.0),
        center: Point3::new(2.0, 2.0, 1.0),
        intensity: 0.5,
    };

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pix = Color::new(0.0, 0.0, 0.0);
            for _s in 0..scene.render_config.samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = scene.camera.get_ray(&mut rng, u, v);
                pix += ray_color(&mut rng, &r, &light, &scene.world, scene.render_config.max_depth);
            }

            print_color(pix, scene.render_config.samples_per_pixel);
        }
    }
}