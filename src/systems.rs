use macroquad::prelude::*;
use crate::Context;


fn start(c:&mut Context) {
    let player = c.state.spawn_actor("guy");
    c.state.me = player.handle;
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

pub fn input_bot(c:&mut Context) {
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
    let s = 32.0;
    draw_rectangle(-s / 2.0, -s / 2.0, s, s, DARKGRAY);

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

fn input_player(c:&mut Context) {
    let mut d = Vec2::new(0.0, 0.0);
    if is_key_down(KeyCode::A) {
        d.x = -1.0;
    }
    if is_key_down(KeyCode::D) {
        d.x = 1.0;
    }
    if is_key_down(KeyCode::W) {
        d.y = -1.0;
    }
    if is_key_down(KeyCode::S) {
        d.y = 1.0;
    }

    let d = d.normalize_or_zero();
    let Some(mut player) = c.state.actor_mut(c.state.me) else { return; };
    player.locomotion = d;
}

fn apply_locomotion(c:&mut Context) {
    let dt = get_frame_time();
    for handle in c.state.actor_handles() {
        let mut actor = c.state.actor_mut(handle).unwrap();
        let speed = actor.info.speed;
        let max_acceleration = speed * speed * dt;
        let desired_vel = actor.locomotion * speed;
        let delta_vel = desired_vel - actor.vel;
        let delta_len = delta_vel.length();
        let delta_dir = delta_vel.normalize_or_zero();
        let add_speed = delta_len.min(max_acceleration);
        actor.vel = actor.vel + delta_dir * add_speed;
    }
}

fn apply_vel(c:&mut Context) {
    let dt = get_frame_time();
    for handle in c.state.actor_handles() {
        let actor = c.state.actor(handle).unwrap();
        let vel = actor.vel;
        let pos = actor.pos;
        let new_pos = pos + vel * dt;

        let mut actor = c.state.actor_mut(handle).unwrap();
        actor.pos = new_pos;
    }
}

pub fn tick(c:&mut Context) {
    let systems = [
        camera,
        input_player,
        input_bot,
        apply_locomotion,
        apply_vel,
        draw
    ];
    for system in systems.iter() {
        system(c);
    }
}