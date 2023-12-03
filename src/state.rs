use glam::Vec2;
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
pub struct Clock {
    pub tick:f32,
    pub current_tick:f32
}
impl Clock {
    pub fn new(tick:f32) -> Self {
        Self {
            tick,
            current_tick:0.0
        }
    }

    pub fn tick(&mut self, dt:f32) -> bool {
        self.current_tick += dt;
        if self.current_tick >= self.tick {
            self.current_tick = 0.0;
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
    pub attack_dir:Vec2
}

#[derive(Default)]
pub struct State {
    pub me:ActorHandle,
    pub actors: SlotMap<ActorHandle, Actor>,
    pub metadata: Rc<Metadata>,
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
