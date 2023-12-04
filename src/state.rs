use glam::{Vec2, Vec4};
use slotmap::{new_key_type, SlotMap};
use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{ActorInfo, AssetIndex, Metadata};

new_key_type! {
    pub struct ActorHandle;
}

#[derive(Default, Clone)]
pub struct Cooldown {
    pub heat:f32
}

impl Cooldown {
    pub fn tick(&mut self, dt:f32) {
        self.heat -= dt;
        if self.heat < 0.0 {
            self.heat = 0.0;
        }
    }

    pub fn activate(&mut self, heat:f32) -> bool {
        if self.heat == 0.0 {
            self.heat = heat;
            return  true;
        }

        return false;
    }
}

#[derive(Default, Clone)]
pub struct Clock {
    pub tick:f32,
}
impl Clock {
    pub fn tick(&mut self, dt:f32, reset_at:f32) -> bool {
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
    pub pos: Vec2,
    pub locomotion_dir: Vec2,
    pub vel: Vec2,
    pub attack_dir:Vec2,
    pub attack_cooldown:Cooldown,
    pub owner:ActorHandle,
    pub health:f32,
    pub color:Vec4
}

#[derive(Clone)]
pub enum ContactEvent {
    Actor {
        actor:ActorHandle,
        other_actor:ActorHandle
    }
}

#[derive(Default)]
pub struct State {
    pub spawner:Clock,
    pub me:ActorHandle,
    pub actors: SlotMap<ActorHandle, Actor>,
    pub metadata: Rc<Metadata>,
    pub contact_events: Vec<ContactEvent>
}

pub struct ActorBorrow<A, B> {
    pub handle: ActorHandle,
    pub actor: A,
    pub info: B,
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
        let actor = Actor {
            info: actor_info.index,
            pos: [0.0, 0.0].into(),
            locomotion_dir: Default::default(),
            vel: Default::default(),
            attack_dir: Default::default(),
            attack_cooldown: Default::default(),
            owner: Default::default(),
            health: actor_info.health,
            color:Vec4::new(1.0, 1.0, 1.0, 1.0)
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
