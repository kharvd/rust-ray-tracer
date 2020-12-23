use crate::vec3::{Point, Vec3, point};
use crate::ray::Ray;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn create(
        lookfrom: &Point,
        lookat: &Point,
        vup: &Vec3,
        vfov_deg: f64,
        aspect_ratio: f64
    ) -> Camera {
        let theta = std::f64::consts::PI / 180.0 * vfov_deg;
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*lookfrom - *lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = *lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray {
            orig: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        };
    }
}
