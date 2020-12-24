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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let scene = scene::read_scene(&args[1])?;
    render_scene(&scene);
    Ok(())
}
