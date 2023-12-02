use std::rc::Rc;

use macroquad::prelude::*;
mod metadata;
pub use metadata::*;
mod context;
pub use context::*;
#[macroquad::main("BasicShapes")]
async fn main() {
    let images = String::from_utf8(load_file("assets/images.toml").await.unwrap()).unwrap();
    let images:toml::Table = images.parse().unwrap();

    let mut metadata = Metadata::default();
    metadata.images.read_from(images).await;
    let mut context = Context::default();
    context.metadata = Rc::new(metadata);
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}