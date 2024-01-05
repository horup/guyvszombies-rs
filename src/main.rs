use std::rc::Rc;
use macroquad::{prelude::*, miniquad::window::set_mouse_cursor};
mod context;
pub use context::*;
mod state;
pub use state::*;
mod systems;
pub use systems::*;
mod infos;
pub use infos::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut context = Context::default();
    context.infos = Infos::new().await;
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