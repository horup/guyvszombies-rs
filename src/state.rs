use glam::{Vec2, Vec4};
use slotmap::{new_key_type, SlotMap};
use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{ActorInfo, AssetIndex, Metadata, WeaponInfo};

new_key_type! {
    pub struct ActorHandle;
}

#[derive(Default, Clone)]
pub struct Timer {
    pub timer: f32,
    pub end_time: f32,
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

#[derive(Default, Clone)]
pub struct Cooldown {
    pub heat: f32,
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

#[derive(Default, Clone)]
pub struct Clock {
    pub tick: f32,
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

#[derive(Default, Clone)]
pub struct Actor {
    pub info: AssetIndex,
    pub weapon: AssetIndex,
    pub weapon_cooldown:f32,
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

impl Actor {
    pub fn facing_vector(&self) -> Vec2 {
        Vec2::new(self.facing.cos(), self.facing.sin())
    }
}

#[derive(Clone)]
pub enum ContactEvent {
    Actor {
        actor: ActorHandle,
        other_actor: ActorHandle,
    },
}

#[derive(Clone)]
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

impl Default for GameState {
    fn default() -> Self {
        Self::Countdown {
            timer: Timer::new(5.0),
        }
    }
}

#[derive(Default)]
pub struct State {
    pub spawner: Clock,
    pub me: ActorHandle,
    pub actors: SlotMap<ActorHandle, Actor>,
    pub metadata: Rc<Metadata>,
    pub contact_events: Vec<ContactEvent>,
    pub round: u32,
    pub game_state: GameState,
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
}

pub struct ActorBorrow<A, B> {
    pub handle: ActorHandle,
    pub actor: A,
    pub info: B,
}

impl<A: Borrow<Actor>, B: Borrow<ActorInfo>> ActorBorrow<A, B> {
    pub fn is_alive(&self) -> bool {
        self.actor.borrow().health > 0.0
    }

    pub fn is_solid(&self) -> bool {
        if !self.is_alive() {
            return false;
        }

        self.info.borrow().solid
    }

    pub fn hand_pos(&self) -> Vec2 {
        let pos = self.pos;
        let v = self.facing_vector();
        pos + v * self.info.borrow().radius
    }

    pub fn muzzle_pos(&self, weapon_info:&WeaponInfo) -> Vec2 {
        let hand = self.hand_pos();
        let v = self.facing_vector();
        hand + v * weapon_info.muzzle_offset
    }
}

impl<A: Borrow<Actor>, B> Deref for ActorBorrow<A, B> {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        self.actor.borrow()
    }
}

impl<A: BorrowMut<Actor>, B> DerefMut for ActorBorrow<A, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.actor.borrow_mut()
    }
}

impl State {
    pub fn despawn_actor(&mut self, handle: ActorHandle) {
        self.actors.remove(handle);
    }
    pub fn spawn_actor(&mut self, info_name: &str) -> ActorBorrow<&mut Actor, &ActorInfo> {
        let actor_info = self
            .metadata
            .actors
            .find(info_name)
            .expect("could not find actor info");
        let weapon = actor_info.weapon;
        let actor = Actor {
            info: actor_info.index,
            pos: [0.0, 0.0].into(),
            locomotion_dir: Default::default(),
            vel: Default::default(),
            attack_dir: Default::default(),
            owner: Default::default(),
            health: actor_info.health,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            pain_timer: Timer::new(0.25),
            frame: 0.0,
            facing: 0.0,
            weapon:weapon,
            weapon_cooldown:0.0
        };
        let handle = self.actors.insert(actor);
        self.actor_mut(handle).unwrap()
    }

    pub fn actor_handles(&self) -> Vec<ActorHandle> {
        self.actors.keys().collect()
    }

    pub fn actor(&self, handle: ActorHandle) -> Option<ActorBorrow<&Actor, &ActorInfo>> {
        let Some(actor) = self.actors.get(handle) else {
            return None;
        };
        let Some(info) = self.metadata.actors.get(actor.info) else {
            return None;
        };
        Some(ActorBorrow {
            handle,
            actor,
            info,
        })
    }

    pub fn actor_mut(
        &mut self,
        handle: ActorHandle,
    ) -> Option<ActorBorrow<&mut Actor, &ActorInfo>> {
        let Some(actor) = self.actors.get_mut(handle) else {
            return None;
        };
        let Some(info) = self.metadata.actors.get(actor.info) else {
            return None;
        };
        Some(ActorBorrow {
            handle,
            actor,
            info,
        })
    }
}
