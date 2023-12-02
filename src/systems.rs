use macroquad::prelude::*;
use crate::Context;


fn start(c:&mut Context) {
    c.state.spawn_actor("guy");

    c.state.spawn_actor("zombie").pos.x = 3.0;
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
    let width = screen_width();
    let height = screen_height();
    let aspect = width /  height;
    let size = 12.0;

    let zoom = 1.0 / size;

    c.camera.zoom = Vec2::new(zoom, zoom * aspect);
}

pub fn bot(c:&mut Context) {
    let dt = get_frame_time();
    for actor in c.state.actor_handles() {
        let Some(mut actor) = c.state.actor_mut(actor) else { continue;} ;
        if actor.info.bot {
            actor.pos.x += dt;
        }
    }
}

pub fn draw(c:&mut Context) {
    set_camera(&c.camera);
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), DARKGRAY);

    for actor in c.state.actor_handles() {
        let Some(actor) = c.state.actor(actor) else { continue;};
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
        bot,
        camera,
        draw
    ];
    for system in systems.iter() {
        system(c);
    }
}