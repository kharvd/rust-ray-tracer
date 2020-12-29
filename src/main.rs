use std::error::Error;

use clap::Clap;

use rust_ray_tracer::render::render_scene;
use rust_ray_tracer::scene;

#[derive(Clap)]
struct Opts {
    scene_file: String,
    output_file: String,

    #[clap(short, long)]
    parallel: bool,

    #[clap(long)]
    no_bvh: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let scene = scene::read_scene(&opts.scene_file)?;
    render_scene(&scene, &opts.output_file, opts.parallel, !opts.no_bvh);
    Ok(())
}
