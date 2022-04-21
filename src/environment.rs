use ruscii::drawing::{Drawable, Pencil};
use ruscii::spatial::Vec2;
use crate::entity::Focusable;

struct Corridor {
    width: i32,
    height: i32,
    material: char,
    focus: Vec2
}

impl Corridor {
    pub fn new() -> Corridor {
        return Corridor { width: 40, height: 10, material: '▒', focus: Vec2::zero() };
    }

    pub fn focus(mut self, pos: &Vec2) {
        self.focus = *pos;
    }
}

impl Drawable for Corridor {
    fn draw(&self, pencil: Pencil) {

    }
}