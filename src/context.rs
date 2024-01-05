use macroquad::camera::Camera2D;

use crate::{State, Metadata};

#[derive(Default)]
pub struct Context {
    pub camera:Camera2D,
    pub metadata:Metadata,
    pub state:State,
    pub debug:bool
}