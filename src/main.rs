mod sprites;


use std::time::{SystemTime, UNIX_EPOCH};
use ruscii::app::{App, Config, State};
use ruscii::drawing::{Drawable, Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

struct Entity {
    sprite: Vec<Vec<char>>,
    color: Color,
    position: Vec2,
    object: Box<dyn BlockItem>
}

trait TEntity {
    fn entity<'a>(&self) -> &'a Entity;
}

impl Entity {
    fn new(object: Box<dyn BlockItem>,sprite_string: &str, position: Vec2, color: Color) -> Self {
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
        return Entity { sprite, position, color, object };
    }
}

impl Drawable for Entity {
    fn draw(&self, mut pencil: Pencil) {
        for y in 0..self.sprite.len() {
            for x in 0..self.sprite[0].len() {
                pencil.set_foreground(self.color);
                pencil.draw_char(self.sprite[y][x], self.position + Vec2::xy(x, y));
            }
        }
    }
}

struct Player {
    entity: Entity,
    hp: i32,
    atk: i32
}

struct Chest {
    entity: Entity,
    amount: i32
}

impl Drawable for Chest {
    fn draw(&self, mut pencil: Pencil) {
        pencil.draw(&self.entity);
    }
}

impl Drawable for Player {
    fn draw(&self, mut pencil: Pencil) {
        pencil.draw(&self.entity);
    }
}

struct Dungeon {
    floors: Vec<Floor>
}

struct Floor {
    blocks: Vec<Block>
}

struct Block {

}

trait BlockItem {

}



fn main() {



    let mut app = App::config(Config::new().fps(60));
    let size = Vec2::xy(80, 30);
    let mut fps_counter = FPSCounter::new();

    let mut last_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to parse duration").as_millis();

    let mut chest_pos = Vec2::xy(5i32, 5i32);
    let mut player_pos = Vec2::xy(15i32, 5i32);

    let chest: Entity = Entity::new(sprites::CHEST, player_pos, Color::Green);
    let player: Entity = Entity::new(sprites::PLAYER, chest_pos, Color::Yellow);


    app.run(|app_state: &mut State, window: &mut Window| {
        let delta_time = calculate_delta(&mut last_time);
        fps_counter.update();

        Pencil::new(window.canvas_mut())
            .draw(&chest)
            .draw(&player)
            .draw_rect(&RectCharset::simple_lines(), Vec2::zero(), size);
    });
}

fn calculate_delta(last_time: &mut u128) -> f32 {
    const LOW_LIMIT: f32 = 0.0167f32;
    const HIGH_LIMIT: f32 = 0.1f32;

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to parse duration").as_millis();
    let mut delta_time: f32 = (current_time - *last_time) as f32 / 1000f32;
    if delta_time < LOW_LIMIT {
        delta_time = LOW_LIMIT;
    } else if delta_time > HIGH_LIMIT {
        delta_time = HIGH_LIMIT;
    }

    *last_time = current_time;
    return delta_time;
}