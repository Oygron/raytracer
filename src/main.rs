mod coord;
mod scene;

use scene::Scene;

fn main() {
    let scene = Scene::load("Scene.xml".to_string());
    scene.render();
}
