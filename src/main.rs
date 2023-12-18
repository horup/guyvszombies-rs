use std::rc::Rc;
use macroquad::{prelude::*, miniquad::window::set_mouse_cursor};
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

    let weapons = String::from_utf8(load_file("assets/weapons.toml").await.unwrap()).unwrap();
    let weapons:toml::Table = weapons.parse().unwrap();
    metadata.weapons.read_from(weapons, &metadata.images).await;


    let mut context = Context::default();
    //context.debug = true;
    context.metadata = Rc::new(metadata);
    context.state.metadata = context.metadata.clone();
    systems::once(&mut context);
    set_mouse_cursor(miniquad::CursorIcon::Crosshair);
    loop {
        systems::tick(&mut context);
        set_default_camera();
        let font_size = 24.0;
        draw_text(&format!("{}", get_fps()), font_size, font_size, font_size, RED);
        next_frame().await
    }
}