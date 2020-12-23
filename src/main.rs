mod vec3;
mod color;
mod ray;

use vec3::Vec3;
use crate::color::{color, print_color};
use crate::ray::Ray;
use crate::vec3::{Color, point};
use crate::sphere::Sphere;

mod sphere {
    use crate::vec3::Point;
    use crate::ray::Ray;

    pub struct Sphere {
        pub center: Point,
        pub radius: f64
    }

    impl Sphere {
        pub fn is_hit_by(&self, ray: &Ray) -> bool {
            let orig_to_center = ray.orig - self.center;
            let a = ray.dir.length2();
            let b = 2.0 * ray.dir.dot(&orig_to_center);
            let c = orig_to_center.length2() - self.radius * self.radius;
            let discr = b * b - 4.0 * a * c;
            return discr >= 0.0;
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    let sphere = Sphere {
        radius: 0.5,
        center: point(0.0, 0.0, -1.0),
    };

    if sphere.is_hit_by(ray) {
        return color(1.0, 0.0, 0.0);
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
                dir: lower_left_corner + u * horizontal + v * vertical - origin
            };
            let pix = ray_color(&r);

            print_color(pix);
        }
    }
}
