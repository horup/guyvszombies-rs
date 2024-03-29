
use std::{f32::consts::PI, io::{Write, Read}};

use crate::{Context, ContactEvent, GameState, Timer, StateSnapshot, State};
use macroquad::prelude::*;


/// Updates the camera based upon the size of the screen, by ensuring zoom is set to the correct level
pub fn camera(c: &mut Context) {
    let width = screen_width();
    let height = screen_height();
    let aspect = width / height;
    let size = 12.0;

    let zoom = 1.0 / size;

    c.camera.zoom = Vec2::new(zoom, zoom * aspect);
}

/// Updates all bots, ensuring their bot logic has run and that the corrosponding bot actors have been updated
pub fn bots(c: &mut Context) {
    for actor in c.state.actor_handles() {
        let Some(bot) = c.state.actor(actor) else {
            continue;
        };
        if !bot.is_alive() {
            continue;
        }
        if !bot.info.bot {
            continue;
        }
        let Some(player) = c.state.actor(c.state.me) else {
            continue;
        };
        let player_is_alive = player.is_alive();
        let v = player.pos - bot.pos;
        let range_to_player = v.length();
        let direction_to_player = v.normalize_or_zero();
       
        let bot = c.state.actor_mut(actor).unwrap();
        if player_is_alive {
            bot.facing = f32::atan2(direction_to_player.y, direction_to_player.x);
            bot.locomotion_dir = direction_to_player;
        } else {
            bot.locomotion_dir = Default::default();
        }
        
        if player_is_alive && range_to_player < bot.weapon.range && player_is_alive {
            bot.attack_dir = v.normalize_or_zero().clone();
        } else {
            bot.attack_dir = Default::default();
        }
    }
}

pub fn draw(c: &mut Context) {
    set_camera(&c.camera);
    let s = 32.0;
    draw_rectangle(-s / 2.0, -s / 2.0, s, s, DARKGRAY);
    let mut sorted_actors = Vec::new();

    for actor in c.state.actor_handles() {
        let Some(actor) = c.state.actor(actor) else {
            continue;
        };
        sorted_actors.push(actor);
    }
    sorted_actors.sort_by(|a, b|a.pos.y.partial_cmp(&b.pos.y).unwrap());


    for actor in sorted_actors.drain(..) {
        let mut frames = &actor.info.frames;
        if actor.locomotion_dir.length() > 0.0 {
            frames = &actor.info.locomotion_frames;
        }
        if actor.health <= 0.0 {
            frames = &actor.info.dead_frames;
        }
        if frames.is_empty() {
            frames = &actor.info.frames;
        }

        if frames.is_empty() {
            continue;
        }
        let f = actor.frame as usize % frames.len();
       
        let frame: &crate::ImageIndex = &frames[f];
        
        let img = &frame.image;
        let texture = &img.texture;
        let size = Vec2::new(2.0, 2.0);
        let x: f32 = actor.pos.x - size.x / 2.0 + actor.info.offset.x;
        let y = actor.pos.y - size.y / 2.0 + actor.info.offset.y;
        let color:[f32;4] = actor.color.into();
        let flip_x = match actor.info.rotate_to_face {
            true => false,
            false => actor.facing_vector().x < 0.0,
        };
        let rotation = match actor.info.rotate_to_face {
            true => actor.facing,
            false => 0.0,
        };
        draw_texture_ex(
            texture,
            x,
            y,
            color.into(),
            DrawTextureParams {
                dest_size: Some(size),
                flip_x,
                rotation,
                ..Default::default()
            },
        );

        // only draw weapons for alive actors
        if actor.is_alive() {
            let weapon_info = actor.weapon.clone();
            let texture = weapon_info.frames.get(0);
            if let Some(frame) = texture {
                let image = &frame.image;
                let v = actor.facing_vector();
                let hand = actor.hand_pos();
                let mount: Vec2 =  hand - size / 2.0 + v * weapon_info.mount_offset * size.length();
                
                draw_texture_ex(&image.texture, mount.x, mount.y, WHITE, DrawTextureParams {
                    dest_size: Some(size),
                    rotation:actor.facing,
                    flip_y:v.x < 0.0,
                    ..Default::default()
                });
    
                if c.debug {
                    let muzzle = actor.muzzle_pos();
                    draw_circle(hand.x, hand.y, 0.1, GREEN);
                    draw_circle(muzzle.x, muzzle.y, 0.1, RED);
                }
            }
        }
    }

}

/// Draw the bounds of the game.
fn draw_bounds(c:&mut Context) {
    let b = c.state.bounds;
    let top_left = c.camera.world_to_screen([b.left, b.top].into());
    let bottom_right = c.camera.world_to_screen([b.right(), b.bottom()].into());
    set_default_camera();
    let color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 };
    draw_rectangle(top_left.x, 0.0, bottom_right.x - top_left.x, top_left.y, color);
    draw_rectangle(top_left.x, bottom_right.y, bottom_right.x - top_left.x, screen_height() - bottom_right.y, color);
    draw_rectangle(0.0, 0.0, top_left.x, screen_height(), color);
    draw_rectangle(bottom_right.x, 0.0, screen_width() - top_left.x, screen_height(), color);
}


pub fn draw_hud(c:&mut Context) {
    set_default_camera();
    let x = screen_width() / 2.0;
    let font_size = 32;
    let y = font_size as f32;
    let s = format!("ROUND {}", &c.state.round);
    let m = measure_text(&s, None, font_size, 1.0);
    draw_text(&s, x - m.width / 2.0, y, font_size as f32, WHITE);

    fn draw_text_center(str:&str, font_size: u16) {
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        let m = measure_text(str, None, font_size, 1.0);
        draw_text(str, x - m.width / 2.0, y, font_size as f32, WHITE);
    }

    match &c.state.game_state {
        GameState::Countdown { timer } => {
            let s = format!("Next round starting in {:.2} seconds", &timer.time_left());
            draw_text_center(&s, font_size);
        },
        GameState::ReadyToRespawn => {
            draw_text_center(&"You died! Click to restart!", font_size);
        }
        _ => {}
    }
}

fn draw_debug(c:&mut Context) {
    if !c.debug { return };
    for actor_handle in c.state.actor_handles() {
        let Some(actor) = c.state.actor(actor_handle) else { continue;};
        let r = actor.info.radius;
        let x = actor.pos.x - r;
        let y = actor.pos.y - r;
        draw_rectangle_lines(x, y, r * 2.0, r * 2.0, 0.1, RED);
        let v = Vec2::new(actor.facing.cos(), actor.facing.sin());
        draw_line(actor.pos.x, actor.pos.y, actor.pos.x + v.x, actor.pos.y + v.y, 0.05, GREEN);
    }

    let b = c.state.bounds;
    draw_rectangle_lines(b.left, b.top, b.width, b.height, 0.1, RED);
}

/// Collects input from the player and update the player actor based upon this input
fn player(c: &mut Context) {
    let mut d = Vec2::new(0.0, 0.0);
    if is_key_pressed(KeyCode::F1) {
        c.debug = !c.debug;
    }
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
    let Some(player) = c.state.actor_mut(c.state.me) else {
        return;
    };


    if player.is_alive() == false {
        return;
    }
    
    if is_key_pressed(KeyCode::Key1) {
        player.weapon = c.metadata.weapons.get("fists").unwrap().clone()
    }
    if is_key_pressed(KeyCode::Key2) {
        player.weapon = c.metadata.weapons.get("pistol").unwrap().clone();
    }
    if is_key_pressed(KeyCode::Key3) {
        player.weapon = c.metadata.weapons.get("machinegun").unwrap().clone();
    }
    if is_key_pressed(KeyCode::Key4) {
        player.weapon = c.metadata.weapons.get("rifle").unwrap().clone();
    }
    if is_key_pressed(KeyCode::Key5) {
    }
    if is_key_pressed(KeyCode::Key6) {
    }
    if is_key_pressed(KeyCode::Key7) {
    }
    if is_key_pressed(KeyCode::Key8) {
    }
    if is_key_pressed(KeyCode::Key9) {
    }
    
    if attack_dir.length() == 0.0 {
        // check mouse
        let m = mouse_position();
        let w = c.camera.screen_to_world(m.into());
        let v = w - player.pos;
        let v = v.normalize_or_zero();
        let a = f32::atan2(v.y, v.x);
        player.facing = a;
        if is_mouse_button_down(MouseButton::Left) {
            let v = v.normalize_or_zero();
            attack_dir = v;
        }
    }

    player.attack_dir = attack_dir;
    player.locomotion_dir = d;
    
}

fn locomotion(c: &mut Context) {
    let dt = get_frame_time();
    for handle in c.state.actor_handles() {
        let actor = c.state.actor_mut(handle).unwrap();
        if !actor.is_alive() {
            actor.locomotion_dir = Vec2::default();
        }
        let speed = actor.info.speed;
        let max_acceleration = speed * speed * dt;
        let desired_vel = actor.locomotion_dir * speed;
        let delta_vel = desired_vel - actor.vel;
        let delta_len = delta_vel.length();
        let delta_dir = delta_vel.normalize_or_zero();
        let add_speed = delta_len.min(max_acceleration);
        actor.vel += delta_dir * add_speed;

        /*if actor.locomotion_dir.length() > 0.0 {
            let d = actor.locomotion_dir.normalize_or_zero();
            let a = f32::atan2(d.y, d.x);
            actor.facing = a;
        }*/
    }
}

/// Update actors position based upon their velocity.
/// 
/// Collects `ContactEvent` for later processing
fn physics(c: &mut Context) {
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

        if actor.is_solid() {
            let shape = parry2d::shape::Cuboid::new([actor.info.radius, actor.info.radius].into());
            let q = spatial.query_around([pos.x, pos.y], 2.0);
            for (handle2,_) in q {
                let handle2 = *spatial.get(handle2).unwrap().1;
                if handle != handle2 {
                    let actor2 = c.state.actor(handle2).unwrap();
                    if !actor2.is_solid() {
                        continue;
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
                    new_pos += push_back;
                    // TODO maybe avoid generating multiple contact events
                    let ce = ContactEvent::Actor { actor: handle, other_actor: handle2 };
                    c.state.contact_events.push(ce);
                }
            }
        }

        let actor = c.state.actor_mut(handle).unwrap();
        actor.pos = new_pos;
    }
}

/// Updates and handle actors whom are attacking with their weapons. 
/// Ensures that projectiles are spawned based upon the attack state.
fn attack(c:&mut Context) {
    let dt = get_frame_time();
    for actor in c.state.actor_handles() {
        let Some(actor) = c.state.actor_mut(actor) else { continue;};
        if actor.is_alive() == false {
            continue;
        }
        actor.weapon_cooldown -= dt;
        if actor.weapon_cooldown < 0.0 {
            actor.weapon_cooldown = 0.0;
        }
        if actor.attack_dir.length() > 0.0 {
            let weapon_info = actor.weapon.clone();
            if actor.weapon_cooldown == 0.0 {
                actor.weapon_cooldown = 1.0 / weapon_info.rate_of_fire;
                if let Some(projectile_actor_info) = c.metadata.actors.get(&weapon_info.projectile) {
                    let speed = projectile_actor_info.velocity;
                    let spawn_pos = actor.muzzle_pos();
                    let spread = rand_f32_1_1() * weapon_info.spread;
                    let facing_with_spread = actor.facing + spread;
                    let d = Vec2::new(facing_with_spread.cos(), facing_with_spread.sin());
                    let v = d * speed;
                    let bullet = c.state.spawn_actor(projectile_actor_info.clone());
                    bullet.pos = spawn_pos;
                    bullet.vel = v;
                    bullet.facing = facing_with_spread;
                }
            }
        }
    }
}

fn rand_f32_0_1() -> f32 {
    let v = macroquad::rand::rand() as f32;
    v / u32::MAX as f32
}

fn rand_f32_1_1() -> f32 {
    let v = rand_f32_0_1();
    let v = v - 0.5;
    v * 2.0
}

/// updates the game_state struct with the current state of the game and
/// ensures transition to other states
pub fn game_state(c:&mut Context) {
    let dt = get_frame_time();
    match &mut c.state.game_state {
        crate::GameState::Countdown { timer } => {
            timer.tick(dt);
            if timer.is_done() {
                c.state.round += 1;
                let i = c.state.round + 1;
                let mobs_to_spawn = i * i;
                c.state.game_state = GameState::Spawning { mobs_left_to_spawn: mobs_to_spawn, mobs_total: mobs_to_spawn };
            }
        },
        crate::GameState::Spawning { mobs_left_to_spawn, mobs_total: _ } => {
            if *mobs_left_to_spawn > 0 {
                *mobs_left_to_spawn -= 1;
                let r = macroquad::rand::rand() / 365;
                let r = r as f32;
                let r = r / 365.0;
                let x = r.cos();
                let y = r.sin();
                let r = 15.0;
                let v = Vec2::new(x * r, y * r);
                c.state.spawn_actor(c.metadata.actors.get("zombie").unwrap().clone()).pos = v;
            } else {
                c.state.game_state = GameState::WaitForDefeat;
            }
        },
        crate::GameState::WaitForDefeat => {
            if c.state.mobs_left() == 0 {
                c.state.game_state = GameState::Countdown { timer: Timer::start(5.0) };
            }
            let Some(player) = c.state.actor(c.state.me) else { return };
            if player.is_alive() == false {
                c.state.game_state = GameState::WaitForReadyToRespawn { timer: Timer::start(1.0) }
            }
        },
        crate::GameState::WaitForReadyToRespawn { timer } => {
            timer.tick(dt);
            if timer.is_done() {
                c.state.game_state = GameState::ReadyToRespawn
            }
        },
        crate::GameState::ReadyToRespawn => {
            if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                start(c);
            }
        }
    }
}

/// Handle missile actors whom are part of `ContactEvent`.
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
                        let min_dmg: f32 = actor.info.missile_direct_damage.0;
                        let max_dmg: f32 = actor.info.missile_direct_damage.1;
                        let dmg = min_dmg + (max_dmg - min_dmg) * rand_f32_0_1();
                        let dmg = dmg.floor();
                        hits.push((other_actor.handle, dmg));
                    }
                    
                    let pos = actor.pos;
                    c.state.despawn_actor(actor.handle);

                    let max = 8;
                    for i in 0..max {
                        let a = i as f32 / max as f32 * PI * 2.0;
                        let v = Vec2::new(a.cos(), a.sin()) * 2.0;
                        let spatter = c.state.spawn_actor(c.metadata.actors.get("spatter").unwrap().clone());
                        spatter.pos = pos;
                        spatter.vel = v;
                    }
                }
            },
        }
    }

    for (actor_handle, dmg) in hits.drain(..) {
        let Some(actor) = c.state.actor_mut(actor_handle) else { continue;};
        actor.health -= dmg;
        let et = actor.pain_timer.end_time;
        actor.pain_timer.restart(et);
    }
}

/// Update particle actors.
/// These are despawned when their health is reduced to zero. 
/// Their alpha color is reduced to zero over time.
fn particle(c:&mut Context) {
    let dt = get_frame_time();
    for actor_handle in c.state.actor_handles() {
        let Some(actor) = c.state.actor_mut(actor_handle) else { continue; };
        if actor.info.particle {
            actor.health -= dt;
            let a = actor.health / actor.info.health;
            actor.color.w = a;
            if !actor.is_alive() {
                c.state.despawn_actor(actor_handle);
            }
        }
    }
}

/// Updates the pain timer of actors.
/// Paints the actor redish based upon the timer value.
fn pain_timer(c:&mut Context) {
    let dt = get_frame_time();
    for actor_handle in c.state.actor_handles() {
        let Some(actor) = c.state.actor_mut(actor_handle) else { continue; }; {
            actor.pain_timer.tick(dt);
            let mut a = actor.pain_timer.alpha();
            if a < 0.5 {
                a = 0.0;
            } else {
                a = 1.0;
            }
            actor.color.y = a;
            actor.color.z = a;
        }
    }
}

/// Updates the frame value of actors.
/// Loops through frames.
fn animation(c:&mut Context) {
    let dt = get_frame_time();
    for actor_handle in c.state.actor_handles() {
        let Some(actor) = c.state.actor_mut(actor_handle) else { continue; };
        actor.frame += 10.0 * dt;
    }
}

/// Persist and Restore `StateSnapshot` to disk. 
fn snapshot(c:&mut Context) {
    if is_key_pressed(KeyCode::F5) {
        let snapshot = StateSnapshot::create_snapshot(&c.state, &c.metadata);
        let bytes = bincode::serialize(&snapshot).unwrap();
        std::fs::File::create("quicksave.sav").unwrap().write_all(&bytes).unwrap();
    }
    else if is_key_pressed(KeyCode::F6) {
        let Ok(mut file) = std::fs::File::open("quicksave.sav") else { return };
        let mut buf = Vec::new();
        let Ok(_size) = file.read_to_end(&mut buf) else { return };
        let snapshot:StateSnapshot = bincode::deserialize(&buf).unwrap();
        c.state = snapshot.load_snapshot(&c.metadata);
    }
}

/// Increment age of actors and despawn the actor if its age reaches max_age (unless max_age is zero)
fn age(c:&mut Context) {
    let dt = get_frame_time();
    for actor_handle in c.state.actor_handles() {
        let actor = c.state.actor_mut(actor_handle).unwrap();
        actor.age += dt;
        if actor.info.max_age > 0.0 && actor.age >= actor.info.max_age {
            c.state.despawn_actor(actor_handle);
        }
    }
}

/// Ensures players are not able to leave the bounds of the game
pub fn player_bounds(c:&mut Context) {
    let b = c.state.bounds;
    if let Some(actor) = c.state.actor_mut(c.state.me){
        actor.pos = actor.pos.clamp([b.left, b.top].into(), [b.right(), b.bottom()].into());
    }
}


/// Clears and starts the game by spawning the player
fn start(c: &mut Context) {
    c.state = State::default();
    let player = c.state.spawn_actor(c.metadata.actors.get("guy").unwrap().clone());
    c.state.me = player.handle;
}

pub fn once(c: &mut Context) {
    let systems = [start];
    for system in systems.iter() {
        system(c);
    }
}

pub fn tick(c: &mut Context) {
    let systems = [
        game_state,
        camera,
        player,
        bots,
        attack,
        locomotion,
        physics,
        player_bounds,
        missile_contact,
        particle,
        pain_timer,
        animation,
        age,
        draw,
        draw_bounds,
        draw_debug,
        draw_hud,
        snapshot
    ];
    for system in systems.iter() {
        system(c);
    }
}
