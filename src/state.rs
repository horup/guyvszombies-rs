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
