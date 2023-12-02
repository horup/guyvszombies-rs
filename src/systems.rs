use macroquad::prelude::*;
use crate::Context;


fn start(c:&mut Context) {
    c.state.spawn_actor("guy");
}

pub fn once(c:&mut Context) {
    let systems = [
        start
    ];

    for system in systems.iter() {
        system(c);
    }
}

pub fn camera(c:&mut Context) {
    let zoom = 0.1;
    c.camera.zoom = Vec2::new(zoom, zoom);
}

pub fn draw(c:&mut Context) {
    set_camera(&c.camera);
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), DARKGRAY);

    for actor in c.state.actor_handles() {
        let Some(actor) = c.state.get_actor(actor) else { continue;};
        let frame = actor.info.frames[0]; 
        let img = c.metadata.images.get(frame.image).unwrap();
        let texture = &img.texture;
       
        let color = WHITE;
        let size = Vec2::new(1.0, 1.0);
        let x = actor.pos.x - size.x / 2.0;
        let y = actor.pos.y / size.y / 2.0;
        draw_texture_ex(texture, x, y, color, DrawTextureParams { dest_size:Some(size), ..Default::default() });
    }
}

pub fn tick(c:&mut Context) {
    let systems = [
        camera,
        draw
    ];
    for system in systems.iter() {
        system(c);
    }
}