use std::rc::Rc;
use macroquad::camera::Camera2D;

use crate::{Metadata, State, Infos};

#[derive(Default)]
pub struct Context {
    pub camera:Camera2D,
    pub metadata:Rc<Metadata>,
    pub infos:Infos,
    pub state:State,
    pub debug:bool
}