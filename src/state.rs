use std::rc::Rc;
use glam::Vec2;
use slotmap::{new_key_type, SlotMap};

use crate::{Metadata, AssetIndex};

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

impl State {
    pub fn spawn_actor(&mut self, actor_info:&str) -> &mut Actor {
        let actor_info = self.metadata.actors.find("guy").expect("could not find actor info");
        let actor = Actor { info:actor_info.index, pos:[0.0, 0.0].into() };
        let handle = self.actors.insert(actor);
        return self.actors.get_mut(handle).unwrap();
    }
}
