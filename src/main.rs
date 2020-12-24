use std::f64;

use crate::camera::Camera;
use crate::color::{color, print_color, random_color};
use crate::geometry::{Hittable, HitRecord, HittableList};
use crate::geometry::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3::{Color, point, Vec3};
use rand::{random, SeedableRng, RngCore, Rng};
use crate::material::{Lambertian, Metal, Dielectric, Material};
use rand::rngs::SmallRng;

mod vec3;
mod color;
mod ray;
mod geometry;
mod camera;
mod material;

fn ray_color(rng: &mut dyn RngCore, ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    let hit_record: Option<HitRecord> = world.hit_by(ray, 0.001, f64::INFINITY);
    return match hit_record {
        Some(rec) => {
            match rec.material.scatter(rng, ray, &rec) {
                Some(scatter_rec) => {
                    scatter_rec.attenuation * ray_color(rng, &scatter_rec.ray, world, depth - 1)
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

fn random_scene(rng: &mut dyn RngCore) -> HittableList {
    let mut world: HittableList = Vec::new();

    world.push(Box::new(Sphere {
        radius: 1000.0,
        center: point(0.0, -1000.0, -1.0),
        material: Box::new(Lambertian {
            albedo: color(0.5, 0.5, 0.5)
        }),
    }));

    let p = point(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = point(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - p).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    let albedo = random_color(rng) * random_color(rng);
                    Box::new(Lambertian {
                        albedo,
                    })
                } else if choose_mat < 0.95 {
                    let albedo = random_color(rng) / 2.0 + 0.5;
                    let fuzz = rng.gen_range(0.0..0.5);
                    Box::new(Metal {
                        albedo,
                        fuzz,
                    })
                } else {
                    Box::new(Dielectric {
                        index_of_refraction: 1.5
                    })
                };

                world.push(Box::new(Sphere {
                    radius: 0.2,
                    center,
                    material,
                }));
            }
        }
    }

    world.push(Box::new(Sphere {
        radius: 1.0,
        center: point(0.0, 1.0, 0.0),
        material: Box::new(Dielectric {
            index_of_refraction: 1.5,
        }),
    }));

    world.push(Box::new(Sphere {
        radius: 1.0,
        center: point(-4.0, 1.0, 0.0),
        material: Box::new(Lambertian {
            albedo: color(0.4, 0.2, 0.1),
        }),
    }));

    world.push(Box::new(Sphere {
        radius: 1.0,
        center: point(4.0, 1.0, 0.0),
        material: Box::new(Metal {
            albedo: color(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    return world;
}

fn main() {
    let mut rng = SmallRng::from_entropy();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene(&mut rng);

    // Camera
    let lookfrom = point(13.0, 2.0, 3.0);
    let lookat = point(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::create(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
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
                let r = camera.get_ray(&mut rng, u, v);
                pix += ray_color(&mut rng, &r, &world, max_depth);
            }

            print_color(pix, samples_per_pixel);
        }
    }
}
