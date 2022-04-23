use ruscii::drawing::{Drawable, Pencil};
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

pub struct Entity {
    name: String,
    sprite: Vec<Vec<char>>,
    color: Color,
    pub position: i32,
    width: i32
}

pub trait TEntity {
    fn entity(&self) -> &Entity;

    fn entity_mut(&mut self) -> &mut Entity;
}

impl Entity {
    pub fn new(name: &str, sprite_string: &str, position: i32, color: Color) -> Self {
        let mut sprite:Vec<Vec<char>> = vec![vec![]];
        let mut y_cord = 0;
        let mut max_width = 0;
        let mut sym_counter = 0;
        for sym in sprite_string.trim_matches('\n').chars() {
            if sym.eq(&'\n') {
                if sym_counter > max_width {
                    max_width = sym_counter;
                }
                sym_counter = 0;
                y_cord+=1;
                sprite.push(vec![]);
                continue;
            }
            sprite[y_cord].push(sym);
            sym_counter+=1;
        }
        return Entity { name: name.to_string(), sprite, position, color, width: max_width };
    }

    pub fn move_by(&mut self, offset: i32) {
        self.position += offset;
    }

    pub fn get_pos(&self) -> i32 {
        self.position
    }

    pub fn collides(&self, other: &Entity) -> bool {
        if (self.position + self.width >= other.position && other.position >= self.position) ||
            (other.position + other.width >= self.position && self.position >= other.position) {
            return true;
        }
        false
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