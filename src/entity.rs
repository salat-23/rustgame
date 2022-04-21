use ruscii::drawing::{Drawable, Pencil};
use ruscii::keyboard::Key;
use ruscii::spatial::Vec2;

pub struct Entity {
    pub x: f32,
    pub y: f32,
    speed: f32,
    character: char
}

impl Entity {
    pub fn new(character: char) -> Entity {
        return Entity { x: 0f32, y: 0f32, speed: 10f32, character }
    }

    pub fn new_at(position: Vec2, character: char) -> Entity {
        return Entity { x: position.x as f32, y: position.y as f32, speed: 10f32, character }
    }
}

pub trait Controllable {
    fn on_pressed(&mut self, key: Key, dt: f32);
}

pub trait Focusable {
    fn get_focus_position(self) -> Vec2;
}

impl Focusable for Entity {
    fn get_focus_position(self) -> Vec2 {
        return Vec2::xy(
            self.x as f32,
            self.y as f32
        );
    }
}

impl Controllable for Entity {
    fn on_pressed(&mut self, key: Key, dt: f32) {
        match key {
            Key::W => { self.y -= self.speed * dt; }
            Key::S => { self.y += self.speed * dt; }
            Key::D => { self.x += self.speed * dt; }
            Key::A => { self.x -= self.speed * dt; }
            _ => ()
        }
    }
}

impl Drawable for Entity {
    fn draw(&self, mut pencil: Pencil) {
        pencil.draw_char(self.character, Vec2::xy(
            self.x as f32,
            self.y as f32
        ));
    }
}