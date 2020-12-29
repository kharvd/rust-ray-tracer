use crate::point3::Point3;
use crate::vec3::Vec3;
use serde::{Serialize, Deserialize};
use crate::color::Color;
use std::{fs, io};
use std::error::Error;
use crate::camera::Camera;
use crate::geometry::Shape;
use rand::{RngCore, Rng, SeedableRng};
use std::fs::File;
use std::io::Write;
use rand::rngs::SmallRng;
use crate::material::Material;
use crate::bvh::BVHNode;
use std::ops::Range;

#[derive(Debug, Serialize, Deserialize)]
struct CameraSpec {
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov_deg: f64,
    aperture: f64,
    focus_dist: f64,
}

impl CameraSpec {
    fn to_camera(&self, render_config: RenderConfig) -> Camera {
        let aspect_ratio = render_config.image_width as f64 / render_config.image_height as f64;
        return Camera::create(
            self.lookfrom,
            self.lookat,
            self.vup,
            self.vfov_deg,
            aspect_ratio,
            self.aperture,
            self.focus_dist,
        );
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RenderConfig {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SceneSpec {
    pub render_config: RenderConfig,
    pub camera: CameraSpec,
    pub objects: Vec<Shape>,
}

impl SceneSpec {
    fn scene(self) -> Scene {
        return Scene {
            render_config: self.render_config,
            camera: self.camera.to_camera(self.render_config),
            shapes: self.objects,
        };
    }
}

pub struct Scene {
    pub render_config: RenderConfig,
    pub camera: Camera,
    pub shapes: Vec<Shape>,
}

impl Scene {
    pub fn bvh(&self, rng: &mut dyn RngCore) -> BVHNode {
        let mut shapes = self.shapes.clone();
        BVHNode::from_shapes(rng, shapes.as_mut_slice())
    }
}

pub fn read_scene(filename: &str) -> Result<Scene, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let scene_spec: SceneSpec = serde_yaml::from_str(contents.as_str())?;
    Ok(scene_spec.scene())
}

fn random_large_scene_spec(rng: &mut dyn RngCore) -> SceneSpec {
    let mut objects: Vec<Shape> = Vec::new();

    objects.push(Shape::SPHERE {
        radius: 1000.0,
        center: Point3::new(0.0, -1000.0, -1.0),
        material: Material::LAMBERTIAN {
            albedo: Color::new(0.5, 0.5, 0.5)
        },
    });

    let p = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - p).length() > 0.9 {
                let material: Material = if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    Material::LAMBERTIAN {
                        albedo,
                    }
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) / 2.0 + 0.5;
                    let fuzz = rng.gen_range(0.0..0.5);
                    Material::METAL {
                        albedo,
                        fuzz,
                    }
                } else {
                    Material::DIELECTRIC {
                        index_of_refraction: 1.5
                    }
                };

                objects.push(Shape::SPHERE {
                    radius: 0.2,
                    center,
                    material,
                });
            }
        }
    }

    objects.push(Shape::SPHERE {
        radius: 1.0,
        center: Point3::new(0.0, 1.0, 0.0),
        material: Material::DIELECTRIC {
            index_of_refraction: 1.5,
        },
    });

    objects.push(Shape::SPHERE {
        radius: 1.0,
        center: Point3::new(-4.0, 1.0, 0.0),
        material: Material::LAMBERTIAN {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    });

    objects.push(Shape::SPHERE {
        radius: 1.0,
        center: Point3::new(4.0, 1.0, 0.0),
        material: Material::METAL {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    return SceneSpec {
        render_config: RenderConfig {
            image_width: 1200,
            image_height: 800,
            samples_per_pixel: 500,
            max_depth: 50,
        },
        camera: CameraSpec {
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            focus_dist: 10.0,
            aperture: 0.1,
            vfov_deg: 20.0,
        },
        objects
    };
}

pub fn setup_small_scene(render_config: RenderConfig) -> Scene {
    let world = vec![
        Shape::PLANE {
            center: Point3::new(0.0, -0.5, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: Material::LAMBERTIAN {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        },
        Shape::SPHERE {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::LAMBERTIAN {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        },
        Shape::SPHERE {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::DIELECTRIC {
                index_of_refraction: 1.5,
            },
        },
        Shape::SPHERE {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Material::DIELECTRIC {
                index_of_refraction: 1.5,
            },
        },
        Shape::SPHERE {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::METAL {
                albedo: Color::new(0.1, 0.2, 0.5),
                fuzz: 0.0,
            },
        },
    ];

    let camera = Camera::create(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        4.0 / 3.0,
        0.0,
        10.0,
    );

    Scene {
        camera,
        shapes: world,
        render_config,
    }
}

fn random_sphere(rng: &mut dyn RngCore, coord_range: Range<f64>, radius_range: Range<f64>) -> Shape {
    let radius = rng.gen_range(radius_range.clone());
    let center = Point3::new(
        rng.gen_range(coord_range.clone()),
        rng.gen_range(coord_range.clone()),
        rng.gen_range(coord_range.clone())
    );

    let material = Material::LAMBERTIAN {
        albedo: Color::new(0.1, 0.2, 0.5),
    };

    Shape::SPHERE {
        radius,
        center,
        material
    }
}

pub fn setup_scene(rng: &mut dyn RngCore, render_config: RenderConfig, num_spheres: u32) -> Scene {
    let mut world = Vec::new();

    for _ in 0..num_spheres {
        world.push(random_sphere(rng, -20.0..20.0, 0.0..0.5));
    }

    let camera = Camera::create(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        4.0 / 3.0,
        0.0,
        10.0,
    );

    Scene {
        camera,
        shapes: world,
        render_config,
    }
}


pub fn random_large_scene(rng: &mut dyn RngCore) -> Scene {
    return random_large_scene_spec(rng).scene()
}

fn _write_scene_spec(filename: &str, scene_spec: &SceneSpec) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(serde_yaml::to_string(scene_spec).unwrap().as_bytes())
}

pub fn _write_large_random_scene(filename: &str) -> Result<(), io::Error> {
    let spec = random_large_scene_spec(&mut SmallRng::from_entropy());
    _write_scene_spec(filename, &spec)
}