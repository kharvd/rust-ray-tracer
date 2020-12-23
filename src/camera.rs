use crate::vec3::{Point, Vec3, point};
use crate::ray::Ray;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn create(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = point(0.0, 0.0, 0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

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
