use std::iter::Rev;
use std::path::is_separator;
use ruscii::drawing::{Drawable, Pencil};
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;

pub struct Sprite {
    sprite: Vec<Vec<char>>,
    color: Color,
    height: i32,
    width: i32,
    is_centered: bool
}

impl Sprite {
    pub fn new(sprite_string: &str, color: Color) -> Sprite {
        let mut sprite: Vec<Vec<char>> = vec![vec![]];
        let mut y_cord = 0;
        let mut max_width = 0;
        let mut sym_counter = 0;
        let mut last_necessary_char = 0;
        for sym in sprite_string.trim_matches('\n').chars() {
            if sym.eq(&'\n') {
                if sym_counter > max_width {
                    max_width = sym_counter;
                }
                sym_counter = 0;
                y_cord += 1;
                sprite.push(vec![]);
                continue;
            }
            sprite[y_cord].push(sym);
            sym_counter += 1;
        }
        Sprite { sprite, color, width: max_width, height: (y_cord + 1) as i32, is_centered: false }
    }

    pub fn centered(mut self) -> Sprite {
        self.is_centered = true;
        self
    }

    pub fn left_top(mut self) -> Sprite {
        self.is_centered = false;
        self
    }

    fn get_reversed(init: char) -> char {
        match init {
            '/' => '\\',
            '\\' => '/',
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            _ => init
        }
    }
}

impl Drawable for Sprite {
    fn draw(&self, mut pencil: Pencil) {
        let mut offset_x = 0;
        let mut offset_y = 0;

        if self.is_centered {
            offset_x = self.width / 2;
            offset_y = self.height / 2;
        }

        let y_start = 0;
        let y_end = self.sprite.len();

        for y in y_start..y_end {
            let mut draw_spaces = false;
            let x_range = 0..self.sprite[y].len();

            for x in x_range {
                let current_char = self.sprite[y][x];
                if current_char != ' ' {
                    draw_spaces = current_char != '®';
                }
                if draw_spaces {
                    pencil.set_foreground(self.color);

                    let x_cord = x as i32 - offset_x;
                    let y_cord = y as i32 - offset_y;

                    let mut drawable_char = current_char;

                    pencil.draw_char(drawable_char, Vec2::xy(x_cord, y_cord));
                }
            }
        }
    }
}

pub struct Entity {
    name: String,
    sprite: Sprite,
    position: i32,
}

pub trait TEntity {
    fn entity(&self) -> &Entity;
    fn entity_mut(&mut self) -> &mut Entity;
}

impl Entity {
    pub fn new(name: &str, sprite_string: &str, position: i32, color: Color) -> Self {
        return Entity { name: name.to_string(), sprite: Sprite::new(sprite_string, color), position };
    }

    pub fn move_by(&mut self, offset: i32) {
        self.position += offset;
    }

    pub fn get_pos(&self) -> i32 {
        self.position
    }

    pub fn collides(&self, other: &Entity) -> bool {
        if (self.position + self.sprite.width >= other.position && other.position >= self.position) ||
            (other.position + other.sprite.width >= self.position && self.position >= other.position) {
            return true;
        }
        false
    }
}

impl Drawable for Entity {
    fn draw(&self, mut pencil: Pencil) {
        pencil.draw(&self.sprite);
    }
}