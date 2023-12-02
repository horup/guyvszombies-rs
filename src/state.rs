use std::rc::Rc;
use glam::Vec2;

use crate::{Metadata, AssetIndex};

#[derive(Default, Clone)]
pub struct Actor {
    pub info:AssetIndex,
    pub pos:Vec2
}

#[derive(Default)]
pub struct State {
    pub metadata:Rc<Metadata>
}

