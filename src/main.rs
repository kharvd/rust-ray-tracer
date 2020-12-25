#[macro_use]
extern crate impl_ops;

use std::env;
use std::error::Error;
use crate::render::render_scene;

mod vec3;
mod color;
mod ray;
mod geometry;
mod camera;
mod material;
mod point3;
mod scene;
mod render;

mod lighting {
    use crate::color::Color;
    use crate::point3::Point3;
    use crate::ray::Ray;

    pub struct IlluminationRecord {
        pub ray: Ray,
        pub color: Color,
        pub t: f64,
        pub intensity: f64,
    }

    pub trait LightSource {
        fn illuminate(&self, point: Point3) -> IlluminationRecord;
    }

    pub struct PointLight {
        pub center: Point3,
        pub color: Color,
        pub intensity: f64,
    }

    impl LightSource for PointLight {
        fn illuminate(&self, point: Point3) -> IlluminationRecord {
            let point_to_center = self.center - point;
            IlluminationRecord {
                ray: Ray {
                    orig: point,
                    dir: point_to_center,
                },
                t: 1.0,
                color: self.color,
                intensity: self.intensity,
            }
        }
    }

    // pub struct GlobalLight {}
    //
    // impl LightSource for GlobalLight {
    //     fn illuminate(&self, point: Point3) -> IlluminationRecord {
    //         let normalized_dir = ray.dir.normalize();
    //         let t = 0.5 * (normalized_dir.1 + 1.0);
    //         (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    //     }
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let scene = scene::read_scene(&args[1])?;
    render_scene(&scene);
    Ok(())
}
