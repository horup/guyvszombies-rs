use glam::{Vec2, Vec4};
use serde::{Serialize, Deserialize};
use slotmap::{new_key_type, SlotMap};
use std::{rc::Rc, ops::{Deref, DerefMut}};

use crate::{ActorInfo, WeaponInfo};

new_key_type! {
    pub struct ActorHandle;
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Timer {
    pub timer: f32,
    pub end_time: f32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Clock {
    pub tick: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorState {
    pub weapon_cooldown: f32,
    pub pos: Vec2,
    pub locomotion_dir: Vec2,
    pub vel: Vec2,
    pub attack_dir: Vec2,
    pub owner: ActorHandle,
    pub health: f32,
    pub color: Vec4,
    pub pain_timer: Timer,
    pub frame: f32,
    pub facing: f32,
}

#[derive(Clone)]
pub struct Actor {
    pub handle: ActorHandle,
    pub info: Rc<ActorInfo>,
    pub weapon: Rc<WeaponInfo>,
    pub state:ActorState
}

#[derive(Default)]
pub struct State {
    pub spawner: Clock,
    pub me: ActorHandle,
    pub actors: SlotMap<ActorHandle, Actor>,
    pub contact_events: Vec<ContactEvent>,
    pub round: u32,
    pub game_state: GameState,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Cooldown {
    pub heat: f32,
}

#[derive(Clone)]
pub enum ContactEvent {
    Actor {
        actor: ActorHandle,
        other_actor: ActorHandle,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameState {
    Countdown {
        timer: Timer,
    },
    Spawning {
        mobs_left_to_spawn: u32,
        mobs_total: u32,
    },
    WaitForDefeat,
}

impl Timer {
    pub fn new(end_time: f32) -> Self {
        Self {
            timer: end_time,
            end_time,
        }
    }

    pub fn start(end_time: f32) -> Self {
        Self {
            timer: 0.0,
            end_time,
        }
    }

    pub fn time_left(&self) -> f32 {
        self.end_time - self.timer
    }

    pub fn restart(&mut self, end_time: f32) {
        self.timer = 0.0;
        self.end_time = end_time;
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        self.timer += dt;
        if self.timer >= self.end_time {
            self.timer = self.end_time;
            return true;
        }

        return false;
    }

    pub fn is_done(&self) -> bool {
        self.timer == self.end_time
    }

    pub fn alpha(&self) -> f32 {
        if self.end_time == 0.0 {
            return 0.0;
        }

        self.timer / self.end_time
    }
}

impl Cooldown {
    pub fn tick(&mut self, dt: f32) {
        self.heat -= dt;
        if self.heat < 0.0 {
            self.heat = 0.0;
        }
    }

    pub fn activate(&mut self, heat: f32) -> bool {
        if self.heat == 0.0 {
            self.heat = heat;
            return true;
        }

        return false;
    }
}

impl Clock {
    pub fn tick(&mut self, dt: f32, reset_at: f32) -> bool {
        self.tick += dt;
        if self.tick > reset_at {
            self.tick = 0.0;
            return true;
        }

        return false;
    }
}

impl Deref for Actor {
    type Target = ActorState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Actor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Actor {
    pub fn facing_vector(&self) -> Vec2 {
        Vec2::new(self.facing.cos(), self.facing.sin())
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_solid(&self) -> bool {
        if !self.is_alive() {
            return false;
        }

        self.info.solid
    }

    pub fn hand_pos(&self) -> Vec2 {
        let pos = self.pos;
        let v = self.facing_vector();
        pos + v * self.info.radius
    }

    pub fn muzzle_pos(&self) -> Vec2 {
        let hand = self.hand_pos();
        let v = self.facing_vector();
        hand + v * self.weapon.muzzle_offset
    }
}


impl Default for GameState {
    fn default() -> Self {
        Self::Countdown {
            timer: Timer::new(5.0),
        }
    }
}

impl State {
    pub fn mobs_left(&self) -> u32 {
        let mut left = 0;
        for actor_handle in self.actor_handles() {
            let Some(actor) = self.actor(actor_handle) else {
                continue;
            };
            if actor.health > 0.0 && actor.info.bot {
                left += 1;
            }
        }

        left
    }

    pub fn despawn_actor(&mut self, handle: ActorHandle) {
        self.actors.remove(handle);
    }

    pub fn spawn_actor(&mut self, actor_info: Rc<ActorInfo>) -> &mut Actor {
        let weapon = actor_info.weapon.clone();
        let handle = self.actors.insert_with_key(|handle| Actor {
            handle,
            state: ActorState {
                health: actor_info.health,
                pos: [0.0, 0.0].into(),
                locomotion_dir: Default::default(),
                vel: Default::default(),
                attack_dir: Default::default(),
                owner: Default::default(),
                color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                pain_timer: Timer::new(0.25),
                frame: 0.0,
                facing: 0.0,
                weapon_cooldown: 0.0,
            },
            info: actor_info,
            weapon: weapon,
        });

        self.actor_mut(handle).unwrap()
    }

    pub fn actor_handles(&self) -> Vec<ActorHandle> {
        self.actors.keys().collect()
    }

    pub fn actor(&self, handle: ActorHandle) -> Option<&Actor> {
        self.actors.get(handle)
    }

    pub fn actor_mut(&mut self, handle: ActorHandle) -> Option<&mut Actor> {
        self.actors.get_mut(handle)
    }
}
