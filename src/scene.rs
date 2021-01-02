use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Range;
use std::sync::Arc;

use itertools::Itertools;
use obj::{load_obj, Obj, Vertex};
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::SmallRng;
use serde::{Deserialize, Serialize};

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::geometry::{ArcHittable, Parallelepiped, Plane, Sphere, Triangle, TriangleMesh};
use crate::material::Material;
use crate::point3::Point3;
use crate::vec3::Vec3;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShapeSpec {
    Sphere {
        center: Point3,
        radius: f64,
        material: Material,
    },
    Plane {
        center: Point3,
        normal: Vec3,
        material: Material,
    },
    Triangle {
        vertices: [Point3; 3],
        material: Material,
    },
    Parallelepiped {
        basis: [Point3; 4],
        material: Material,
    },
    Object {
        filename: String,
        material: Material,
    },
}

fn read_obj(filename: &String, material: Material) -> TriangleMesh {
    let input = BufReader::new(File::open(filename).unwrap());
    let object: Obj<Vertex, usize> = load_obj(input).unwrap();
    let vertices =
        object.vertices.iter()
            .map(|v|
                Point3::new(
                    v.position[0] as f64,
                    v.position[1] as f64,
                    v.position[2] as f64,
                )
            )
            .collect_vec();
    TriangleMesh::new(&vertices, &object.indices, material)
}

impl ShapeSpec {
    fn to_hittable(&self) -> ArcHittable {
        match self {
            ShapeSpec::Sphere { center, radius, material } => Arc::new(Sphere {
                center: *center,
                radius: *radius,
                material: *material,
            }),
            ShapeSpec::Plane { center, normal, material } => Arc::new(Plane {
                center: *center,
                normal: *normal,
                material: *material,
            }),
            ShapeSpec::Triangle { vertices, material } => Arc::new(Triangle {
                vertices: *vertices,
                material: *material,
            }),
            ShapeSpec::Parallelepiped { basis, material } => Arc::new(
                Parallelepiped::new(basis[0], basis[1], basis[2], basis[3], *material)
            ),
            ShapeSpec::Object { filename, material } => Arc::new(
                read_obj(filename, *material)
            )
        }
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
    pub objects: Vec<ShapeSpec>,
}

impl SceneSpec {
    fn scene(self) -> Scene {
        return Scene {
            render_config: self.render_config,
            camera: self.camera.to_camera(self.render_config),
            hittables: self.objects.iter().map(|o| o.to_hittable()).collect_vec(),
        };
    }
}

pub struct Scene {
    pub render_config: RenderConfig,
    pub camera: Camera,
    pub hittables: Vec<ArcHittable>,
}

impl Scene {
    pub fn bvh(&self, rng: &mut dyn RngCore) -> BVHNode<ArcHittable> {
        let mut shapes = self.hittables.clone();
        BVHNode::from_shapes(rng, shapes.as_mut_slice())
    }
}

pub fn read_scene(filename: &str) -> Result<Scene, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let scene_spec: SceneSpec = serde_yaml::from_str(contents.as_str())?;
    Ok(scene_spec.scene())
}

fn random_large_scene_spec(rng: &mut dyn RngCore) -> SceneSpec {
    let mut objects: Vec<ShapeSpec> = Vec::new();

    objects.push(ShapeSpec::Sphere {
        radius: 1000.0,
        center: Point3::new(0.0, -1000.0, -1.0),
        material: Material::Lambertian {
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
                    Material::Lambertian {
                        albedo,
                    }
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) / 2.0 + 0.5;
                    let fuzz = rng.gen_range(0.0..0.5);
                    Material::Metal {
                        albedo,
                        fuzz,
                    }
                } else {
                    Material::Dielectric {
                        index_of_refraction: 1.5
                    }
                };

                objects.push(ShapeSpec::Sphere {
                    radius: 0.2,
                    center,
                    material,
                });
            }
        }
    }

    objects.push(ShapeSpec::Sphere {
        radius: 1.0,
        center: Point3::new(0.0, 1.0, 0.0),
        material: Material::Dielectric {
            index_of_refraction: 1.5,
        },
    });

    objects.push(ShapeSpec::Sphere {
        radius: 1.0,
        center: Point3::new(-4.0, 1.0, 0.0),
        material: Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    });

    objects.push(ShapeSpec::Sphere {
        radius: 1.0,
        center: Point3::new(4.0, 1.0, 0.0),
        material: Material::Metal {
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
        objects,
    };
}

pub fn setup_small_scene(render_config: RenderConfig) -> Scene {
    let world = vec![
        ShapeSpec::Plane {
            center: Point3::new(0.0, -0.5, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: Material::Lambertian {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        },
        ShapeSpec::Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Lambertian {
                albedo: Color::new(0.1, 0.2, 0.5),
            },
        },
        ShapeSpec::Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Dielectric {
                index_of_refraction: 1.5,
            },
        },
        ShapeSpec::Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Material::Dielectric {
                index_of_refraction: 1.5,
            },
        },
        ShapeSpec::Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: Color::new(0.1, 0.2, 0.5),
                fuzz: 0.0,
            },
        },
    ].iter().map(|s| s.to_hittable()).collect_vec();

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
        hittables: world,
        render_config,
    }
}

fn random_sphere(rng: &mut dyn RngCore, coord_range: Range<f64>, radius_range: Range<f64>) -> Sphere {
    let radius = rng.gen_range(radius_range.clone());
    let center = Point3::new(
        rng.gen_range(coord_range.clone()),
        rng.gen_range(coord_range.clone()),
        rng.gen_range(coord_range.clone()),
    );

    let material = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };

    Sphere {
        radius,
        center,
        material,
    }
}

pub fn setup_scene(rng: &mut dyn RngCore, render_config: RenderConfig, num_spheres: u32) -> Scene {
    let mut world: Vec<ArcHittable> = Vec::new();

    for _ in 0..num_spheres {
        world.push(Arc::new(random_sphere(rng, -20.0..20.0, 0.0..0.5)));
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
        hittables: world,
        render_config,
    }
}


pub fn random_large_scene(rng: &mut dyn RngCore) -> Scene {
    return random_large_scene_spec(rng).scene();
}

fn _write_scene_spec(filename: &str, scene_spec: &SceneSpec) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(serde_yaml::to_string(scene_spec).unwrap().as_bytes())
}

pub fn _write_large_random_scene(filename: &str) -> Result<(), io::Error> {
    let spec = random_large_scene_spec(&mut SmallRng::from_entropy());
    _write_scene_spec(filename, &spec)
}