use crate::{Context, ContactEvent};
use macroquad::prelude::*;

fn start(c: &mut Context) {
    let player = c.state.spawn_actor("guy");
    c.state.me = player.handle;
}

pub fn once(c: &mut Context) {
    let systems = [start];
    for system in systems.iter() {
        system(c);
    }
}

pub fn camera(c: &mut Context) {
    let width = screen_width();
    let height = screen_height();
    let aspect = width / height;
    let size = 12.0;

    let zoom = 1.0 / size;

    c.camera.zoom = Vec2::new(zoom, zoom * aspect);
}

pub fn input_bot(c: &mut Context) {
    //let dt: f32 = get_frame_time();
    for actor in c.state.actor_handles() {
        let Some(mut bot) = c.state.actor(actor) else {
            continue;
        };
        if bot.info.bot == false {
            continue;
        }
        let Some(player) = c.state.actor(c.state.me) else {
            continue;
        };

        let v = player.pos - bot.pos;
        let d = v.normalize_or_zero();
        let mut bot = c.state.actor_mut(actor).unwrap();
        bot.locomotion_dir = d;
    }
}

pub fn draw(c: &mut Context) {
    set_camera(&c.camera);
    let s = 32.0;
    draw_rectangle(-s / 2.0, -s / 2.0, s, s, DARKGRAY);

    for actor in c.state.actor_handles() {
        let Some(actor) = c.state.actor(actor) else {
            continue;
        };
        let frame = actor.info.frames[0];
        let img = c.metadata.images.get(frame.image).unwrap();
        let texture = &img.texture;

        let color = WHITE;
        let size = Vec2::new(1.0, 1.0);
        let x = actor.pos.x - size.x / 2.0;
        let y = actor.pos.y - size.y / 2.0;
        draw_texture_ex(
            texture,
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
}

fn input_player(c: &mut Context) {
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

    let mut attack_dir = Vec2::new(0.0, 0.0);
    if is_key_down(KeyCode::Left) {
        attack_dir.x = -1.0;
    }
    if is_key_down(KeyCode::Right) {
        attack_dir.x = 1.0;
    }
    if is_key_down(KeyCode::Up) {
        attack_dir.y = -1.0;
    }
    if is_key_down(KeyCode::Down) {
        attack_dir.y = 1.0;
    }

    let d = d.normalize_or_zero();
    let Some(mut player) = c.state.actor_mut(c.state.me) else {
        return;
    };
    player.attack_dir = attack_dir;
    player.locomotion_dir = d;
}

fn apply_locomotion(c: &mut Context) {
    let dt = get_frame_time();
    for handle in c.state.actor_handles() {
        let mut actor = c.state.actor_mut(handle).unwrap();
        let speed = actor.info.speed;
        let max_acceleration = speed * speed * dt;
        let desired_vel = actor.locomotion_dir * speed;
        let delta_vel = desired_vel - actor.vel;
        let delta_len = delta_vel.length();
        let delta_dir = delta_vel.normalize_or_zero();
        let add_speed = delta_len.min(max_acceleration);
        actor.vel = actor.vel + delta_dir * add_speed;
    }
}

fn apply_vel(c: &mut Context) {
    c.state.contact_events.clear();
    let mut actor_handles = c.state.actor_handles();
    let mut spatial = flat_spatial::Grid::new(1);
    for handle in actor_handles.iter() {
        let pos = c.state.actor(*handle).unwrap().pos;
        spatial.insert([pos.x, pos.y], *handle);
    }
    // TODO apply substeps
    let dt = get_frame_time();
    for handle in actor_handles.drain(..) {
        let actor = c.state.actor(handle).unwrap();
        let vel = actor.vel;
        if vel.length() == 0.0 { continue; };
        let pos = actor.pos;
        let mut new_pos = pos + vel * dt;

        if actor.info.solid {
            let shape = parry2d::shape::Cuboid::new([actor.info.radius, actor.info.radius].into());
            let q = spatial.query_around([pos.x, pos.y], 2.0);
            for (handle2,_) in q {
                let handle2 = *spatial.get(handle2).unwrap().1;
                if handle != handle2 {
                    let actor2 = c.state.actor(handle2).unwrap();
                    if actor2.info.solid == false {
                        continue;;
                    }
                    let v = actor2.pos - pos;
                    let v = v.normalize_or_zero();
                    if v.dot(vel) < 0.0 { continue;};

                    let shape2 =
                        parry2d::shape::Cuboid::new([actor2.info.radius, actor2.info.radius].into());

                    
                    let contact = parry2d::query::contact(
                        &[new_pos.x, new_pos.y].into(),
                        &shape,
                        &[actor2.pos.x, actor2.pos.y].into(),
                        &shape2,
                        2.0,
                    );

                    let Ok(contact) = contact else {
                        continue;
                    };
                    let Some(contact) = contact else {
                        continue;
                    };

                    if contact.dist > 0.0 {
                        continue;
                    };

                    let push_back = Vec2::new(contact.normal1.x, contact.normal1.y) * contact.dist;
                    new_pos = new_pos + push_back;
                    // TODO maybe avoid generating multiple contact events
                    let ce = ContactEvent::Actor { actor: handle, other_actor: handle2 };
                    c.state.contact_events.push(ce);
                }
            }
        }

        let mut actor = c.state.actor_mut(handle).unwrap();
        actor.pos = new_pos;
    }
}

fn attack(c:&mut Context) {
    let dt = get_frame_time();
    for actor in c.state.actor_handles() {
        let Some(mut actor) = c.state.actor_mut(actor) else { continue;};
        actor.attack_cooldown.tick(dt);
        if actor.attack_dir.length() > 0.0 {
            if actor.attack_cooldown.activate(0.2) {
                let pos = actor.pos;
                let d = actor.attack_dir;
                let r = actor.info.radius + 0.3;
                let speed = 20.0;
                let spawn_pos = pos + d * r;
                let v = d * speed;
                let mut bullet = c.state.spawn_actor("bullet");
                bullet.pos = spawn_pos;
                bullet.vel = v;
            }
        }
    }
}

pub fn spawner(c:&mut Context) {
    let dt = get_frame_time();
    if c.state.spawner.tick(dt, 1.0) {
        let r = macroquad::rand::rand() / 365;
        let r = r as f32;
        let r = r / 365.0;
        let x = r.cos();
        let y = r.sin();
        let r = 8.0;
        let v = Vec2::new(x * r, y * r);
        c.state.spawn_actor("zombie").pos = v;
    }
}

pub fn missile_contact(c:&mut Context) {
    let contacts = c.state.contact_events.clone();
    let mut hits = Vec::new();
    for ev in contacts.iter() {
        match ev {
            ContactEvent::Actor { actor, other_actor } => {
                let Some(actor) = c.state.actor(*actor) else { continue;};
                if actor.info.missile {
                    let Some(other_actor) = c.state.actor(*other_actor) else { continue;};
                    if other_actor.info.shootable {
                        hits.push((other_actor.handle, 10.0));
                    }
                    
                    c.state.despawn_actor(actor.handle);
                }
            },
        }
    }

    for (actor, dmg) in hits.drain(..) {
        let Some(mut actor) = c.state.actor_mut(actor) else { continue;};
        dbg!(actor.health);
    }
}

pub fn tick(c: &mut Context) {
    let systems = [
        spawner,
        camera,
        input_player,
        input_bot,
        attack,
        apply_locomotion,
        apply_vel,
        missile_contact,
        draw,
    ];
    for system in systems.iter() {
        system(c);
    }
}
