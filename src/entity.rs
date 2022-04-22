use ruscii::drawing::{Drawable, Pencil};
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

pub struct Entity {
    name: String,
    sprite: Vec<Vec<char>>,
    color: Color,
    pub position: i32
}

pub trait TEntity {
    fn entity(&self) -> &Entity;
}

impl Entity {
    pub fn new(name: &str, sprite_string: &str, position: i32, color: Color) -> Self {
        let mut sprite:Vec<Vec<char>> = vec![vec![]];
        let mut y_cord = 0;
        for sym in sprite_string.trim_matches('\n').chars() {
            if sym.eq(&'\n') {
                y_cord+=1;
                sprite.push(vec![]);
                continue;
            }
            sprite[y_cord].push(sym);
        }
        return Entity { name: name.to_string(), sprite, position, color };
    }

    pub fn get_pos(&self) -> i32 {
        self.position
    }
}

impl Drawable for Entity {
    fn draw(&self, mut pencil: Pencil) {
        for y in 0..self.sprite.len() {
            for x in 0..self.sprite[0].len() {
                pencil.set_foreground(self.color);
                pencil.draw_char(self.sprite[y][x], Vec2::xy(x, y));
            }
        }
    }
}