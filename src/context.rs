use std::rc::Rc;
use macroquad::camera::Camera2D;

use crate::{Metadata, State};

#[derive(Default)]
pub struct Context {
    pub camera:Camera2D,
    pub metadata:Rc<Metadata>,
    pub state:State,
    pub debug:bool
}