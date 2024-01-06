use macroquad::{prelude::*, miniquad::window::set_mouse_cursor};
mod context;
pub use context::*;
mod state;
pub use state::*;
mod systems;
pub use systems::*;
mod metadata;
pub use metadata::*;
mod snapshot;
pub use snapshot::*;
mod arena;
pub use arena::*;

#[macroquad::main("Guy vs Zombies!")]
async fn main() {
    let mut context = Context::default();
    context.metadata = Metadata::new().await;
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