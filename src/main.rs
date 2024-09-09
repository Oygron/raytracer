use clap::Parser;
use scene::{Parallel, Scene};

mod coord;
mod scene;

/// Generates an image from a scene with a raytracing algorithm
#[derive(Parser, Debug)]
#[command(name = "Raytracer")]
#[command(version = "1.0")]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "scene.xml")]
    input: String,

    #[arg(short, long, default_value = "image.png")]
    output: String,

    #[arg(short, long, default_value = "no")]
    parallel: Parallel,
}

fn main() {
    let args = Args::parse();

    let scene = Scene::load(args.input);
    scene.render(args.parallel, args.output);
}
