use std::rc::Rc;
use macroquad::prelude::*;
mod metadata;
pub use metadata::*;
mod context;
pub use context::*;
mod state;
pub use state::*;
mod systems;
pub use systems::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let images = String::from_utf8(load_file("assets/images.toml").await.unwrap()).unwrap();
    let images:toml::Table = images.parse().unwrap();
    let mut metadata = Metadata::default();
    metadata.images.read_from(images).await;

    let actors = String::from_utf8(load_file("assets/actors.toml").await.unwrap()).unwrap();
    let actors:toml::Table = actors.parse().unwrap();
    metadata.actors.read_from(actors, &metadata.images).await;
    let mut context = Context::default();
    context.metadata = Rc::new(metadata);
    context.state.metadata = context.metadata.clone();
    systems::once(&mut context);
    loop {
        systems::tick(&mut context);
        set_default_camera();
        draw_text(&format!("{}", get_fps()), 12.0, 12.0, 16.0, RED);
        next_frame().await
    }
}