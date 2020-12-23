use std::f64;

use vec3::Vec3;

use crate::camera::Camera;
use crate::color::{color, print_color};
use crate::geometry::{HitRecord, Hittable};
use crate::geometry::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::{Color, point};

mod vec3;
mod color;
mod ray;
mod geometry;
mod camera;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let hit_record = world.hit_by(ray, 0.0, f64::INFINITY);
    return match hit_record {
        Some(hit) =>
            0.5 * color(hit.normal.0 + 1.0, hit.normal.1 + 1.0, hit.normal.2 + 1.0),

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
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = camera.get_ray(u, v);
            let pix = ray_color(&r, &world);

            print_color(pix);
        }
    }
}
