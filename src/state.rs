use std::{rc::Rc, borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}};
use glam::Vec2;
use slotmap::{new_key_type, SlotMap};

use crate::{Metadata, AssetIndex, ActorInfo};

new_key_type! {
    pub struct ActorHandle;
}

#[derive(Default, Clone)]
pub struct Actor {
    pub info:AssetIndex,
    pub pos:Vec2
}

#[derive(Default)]
pub struct State {
    pub actors:SlotMap<ActorHandle, Actor>,
    pub metadata:Rc<Metadata>
}


pub struct ActorBorrow<A, B> {
    pub handle:ActorHandle,
    pub actor:A,
    pub info:B
}

impl<A:Borrow<Actor>, B> Deref for ActorBorrow<A, B> {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        self.actor.borrow()
    }
}

impl<A:BorrowMut<Actor>, B> DerefMut for ActorBorrow<A, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.actor.borrow_mut()
    }
}

impl State {
    pub fn spawn_actor(&mut self, info_name:&str) -> &mut Actor {
        let actor_info = self.metadata.actors.find(info_name).expect("could not find actor info");
        let actor = Actor { info:actor_info.index, pos:[0.0, 0.0].into() };
        let handle = self.actors.insert(actor);
        return self.actors.get_mut(handle).unwrap();
    }

    pub fn actor_handles(&self) -> Vec<ActorHandle> {
        self.actors.keys().collect()
    }

    pub fn actor(&self, handle:ActorHandle) -> Option<ActorBorrow<&Actor, &ActorInfo>> {
        let Some(actor) = self.actors.get(handle) else { return None; };
        let Some(info) = self.metadata.actors.get(actor.info) else { return None; };
        Some(ActorBorrow {
            handle,
            actor,
            info
        })
    }

    pub fn actor_mut(&mut self, handle:ActorHandle) -> Option<ActorBorrow<&mut Actor, &ActorInfo>> {
        let Some(actor) = self.actors.get_mut(handle) else { return None; };
        let Some(info) = self.metadata.actors.get(actor.info) else { return None; };
        Some(ActorBorrow {
            handle,
            actor,
            info
        })
    }
}
