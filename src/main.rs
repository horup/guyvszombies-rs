use std::rc::Rc;

use macroquad::prelude::*;
mod metadata;
pub use metadata::*;
mod context;
pub use context::*;
mod state;
pub use state::*;

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
    loop {
        clear_background(DARKGREEN);

        draw_texture(&context.metadata.images.find("guy").unwrap().texture, 10.0, 10.0, WHITE);

        next_frame().await
    }
}