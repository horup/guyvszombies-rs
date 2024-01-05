use macroquad::camera::Camera2D;

use crate::{State, Infos};

#[derive(Default)]
pub struct Context {
    pub camera:Camera2D,
    pub infos:Infos,
    pub state:State,
    pub debug:bool
}