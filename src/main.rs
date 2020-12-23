use std::f64;

use vec3::Vec3;

use crate::camera::Camera;
use crate::color::{color, print_color};
use crate::geometry::{HitRecord, Hittable};
use crate::geometry::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::{Color, point};
use rand::random;

mod vec3;
mod color;
mod ray;
mod geometry;
mod camera;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    let hit_record = world.hit_by(ray, 0.0, f64::INFINITY);
    return match hit_record {
        Some(rec) => {
            let target = rec.point + rec.normal + Vec3::random_in_unit_sphere();
            let next_ray = Ray { orig: rec.point, dir: target - rec.point };
            0.5 * ray_color(&next_ray, world, depth - 1)
        }

        _ => {
            let normalized_dir = ray.dir.normalize();
            let t = 0.5 * (normalized_dir.1 + 1.0);
            (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0)
        }
    };
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 10;

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            radius: 0.5,
            center: point(0.0, 0.0, -1.0),
        }),
        Box::new(Sphere {
            radius: 100.0,
            center: point(0.0, -100.5, -1.0),
        })
    ];

    // Camera
    let camera = Camera::create(16.0 / 9.0, 2.0, 1.0);

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pix = color(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pix += ray_color(&r, &world, max_depth);
            }

            print_color(pix, samples_per_pixel);
        }
    }
}
