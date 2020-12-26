use crate::geometry::Hittable;
use crate::camera::Camera;
use crate::geometry::Hittable::{PLANE, SPHERE};
use crate::point3::Point3;
use crate::vec3::Vec3;
use crate::material::Material;
use crate::color::Color;

pub fn setup_small_scene() -> (Vec<Hittable>, Camera) {
    let shapes = vec![
        PLANE {
            center: Point3(0.0, -0.5, 0.0),
            normal: Vec3(0.0, 1.0, 0.0),
            material: Material::LAMBERTIAN {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        },
        SPHERE {
            center: Point3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::LAMBERTIAN {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        },
        SPHERE {
            center: Point3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::DIELECTRIC {
                index_of_refraction: 1.5,
            },
        },
        SPHERE {
            center: Point3(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Material::DIELECTRIC {
                index_of_refraction: 1.5,
            },
        },
        SPHERE {
            center: Point3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::METAL {
                albedo: Color::new(0.1, 0.2, 0.5),
                fuzz: 0.0,
            },
        },
    ];

    let camera = Camera::create(
        Point3(-2.0, 2.0, 1.0),
        Point3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        90.0,
        4.0 / 3.0,
        0.0,
        10.0,
    );
    (shapes, camera)
}


