use std::f64;

use crate::camera::Camera;
use crate::color::{color, print_color};
use crate::geometry::Hittable;
use crate::geometry::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::{Color, point, Vec3};
use rand::random;
use std::rc::Rc;
use crate::material::{Lambertian, Metal, Dielectric};

mod vec3;
mod color;
mod ray;
mod geometry;
mod camera;
mod material;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    let hit_record = world.hit_by(ray, 0.001, f64::INFINITY);
    return match hit_record {
        Some(rec) => {
            match rec.material.scatter(ray, &rec) {
                Some(scatter_rec) => {
                    scatter_rec.attenuation * ray_color(&scatter_rec.ray, world, depth - 1)
                }

                None => color(0.0, 0.0, 0.0)
            }
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
    let material_ground = Rc::new(Lambertian {
        albedo: color(0.8, 0.8, 0.0)
    });
    let material_center = Rc::new(Lambertian {
        albedo: color(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Dielectric {
        index_of_refraction: 1.5,
    });
    let material_right = Rc::new(Metal {
        albedo: color(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            radius: 0.5,
            center: point(0.0, 0.0, -1.0),
            material: material_center,
        }),
        Box::new(Sphere {
            radius: 0.5,
            center: point(-1.0, 0.0, -1.0),
            material: material_left.clone(),
        }),
        Box::new(Sphere {
            radius: -0.45,
            center: point(-1.0, 0.0, -1.0),
            material: material_left.clone(),
        }),
        Box::new(Sphere {
            radius: 0.5,
            center: point(1.0, 0.0, -1.0),
            material: material_right,
        }),
        Box::new(Sphere {
            radius: 100.0,
            center: point(0.0, -100.5, -1.0),
            material: material_ground,
        })
    ];

    // Camera
    let lookfrom = point(3.0, 3.0, 2.0);
    let lookat = point(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.2;
    let camera = Camera::create(
        &lookfrom,
        &lookat,
        &Vec3(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pix = color(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pix += ray_color(&r, &world, max_depth);
            }

            print_color(pix, samples_per_pixel);
        }
    }
}
