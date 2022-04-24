use std::ops::Deref;
use ruscii::drawing::{Drawable, Pencil};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Color;
use crate::entities::chest::Chest;
use crate::entities::creature::{Creature, Player, TCreature};
use crate::entities::slime::Slime;
use crate::Entity;
use crate::entity::TEntity;
use crate::sprites::PLAYER;
use crate::stats::Stats;

pub struct Dungeon {
    player: Player,
    current_floor:
    enemies: Vec<Box<dyn TCreature>>,

}

impl Dungeon {
    pub fn generate() -> Dungeon {
        let mut player = Player::new();
        let mut enemies: Vec<Box<dyn TCreature>> = Vec::new();

        for i in 1..
        enemies.push(Box::new(Slime::new(-10)));

        Dungeon { player, enemies }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn enemies(&self) -> &Vec<Box<dyn TCreature>> {
        &self.enemies
    }

    pub fn update(&mut self, delta_time: f32, key_events: &Vec<KeyEvent>) {
        for event in key_events {
            match event {
                KeyEvent::Pressed(Key::A) => self.player_mut().creature_mut().entity_mut().move_by(-1i32),
                KeyEvent::Pressed(Key::D) => self.player_mut().creature_mut().entity_mut().move_by(1i32),
                _ => ()
            }
        }
    }
}

impl Drawable for Dungeon {
    fn draw(&self, mut pencil: Pencil) {
        pencil
            .draw_hline('#', Vec2::y(10i32), 80i32)
            .draw_hline('#', Vec2::y(20i32), 80i32);

        let player_pos = self.player().creature().entity().get_pos();

        for entity in self.enemies().iter() {

            if self.player.creature().entity().collides(entity.creature().entity()) {
                pencil.draw_text("COLLIDES", Vec2::xy(1, 2));
            }

            let ent_pos = entity.creature().entity().get_pos();
            for i in player_pos - 50..player_pos + 50 {
                if i == ent_pos {
                    pencil.draw_at(entity.creature().entity(), Vec2::xy(40i32 + (ent_pos - player_pos), 15i32));
                }
            }
        }
        pencil.draw_at(self.player().creature().entity(), Vec2::xy(40i32, 15i32));

        pencil.draw_text((String::from("Player pos: ")
            + self.player.creature().entity().get_pos().to_string().as_str()).as_str(),
                         Vec2::xy(1i32, 1i32));

    }
}