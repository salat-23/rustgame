use ruscii::drawing::{Drawable, Pencil};
use crate::entity::Entity;

pub struct Chest {
    entity: Entity,
    amount: i32
}

impl Drawable for Chest {
    fn draw(&self, mut pencil: Pencil) {
        pencil.draw(&self.entity);
    }
}