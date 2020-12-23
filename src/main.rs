mod vec3;
mod color;
mod ray;
mod geometry;

use vec3::Vec3;
use crate::color::{color, print_color};
use crate::ray::Ray;
use crate::vec3::{Color, point};
use crate::geometry::sphere::Sphere;
use crate::geometry::Hittable;

fn ray_color(ray: &Ray) -> Color {
    let sphere = Sphere {
        radius: 0.5,
        center: point(0.0, 0.0, -1.0),
    };

    let t = sphere.hit_by(ray);
    if t > 0.0 {
        let normal = sphere.normal_at(&ray.at(t));
        return 0.5 * color(normal.0 + 1.0, normal.1 + 1.0, normal.2 + 1.0);
    }

    let normalized_dir = ray.dir.normalize();
    let t = 0.5 * (normalized_dir.1 + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pix = ray_color(&r);

            print_color(pix);
        }
    }
}
