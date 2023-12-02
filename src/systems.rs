use macroquad::prelude::*;
use crate::Context;


fn start(c:&mut Context) {

}

pub fn once(c:&mut Context) {
    let systems = [
        start
    ];

    for system in systems.iter() {
        system(c);
    }
}

pub fn draw(c:&mut Context) {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), DARKGRAY);
}

pub fn tick(c:&mut Context) {
    let systems = [
        draw
    ];
    for system in systems.iter() {
        system(c);
    }
}