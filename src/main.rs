use std::env;
use std::error::Error;
use rust_ray_tracer::scene;
use rust_ray_tracer::render::render_scene;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let scene = scene::read_scene(&args[1])?;
    render_scene(&scene);
    Ok(())
}
