use ruscii::drawing::{Drawable, Pencil};
use ruscii::keyboard::KeyEvent;
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
    enemies: Vec<Box<dyn TCreature>>,
}

impl<'a> Dungeon {
    pub fn generate() -> Dungeon {
        let mut player = Player::new();
        let mut enemies: Vec<Box<dyn TCreature>> = Vec::new();
        enemies.push(Box::new(Slime::new(-10)));

        Dungeon { player, enemies }
    }

  pub fn player(&self) -> &Player {
      &self.player
  }

    pub fn enemies(&self) -> &Vec<Box<dyn TCreature>> {
        &self.enemies
    }

    pub fn update(&self, delta_time: f32, key_events: Vec<KeyEvent>) {}
}

impl Drawable for Dungeon {
    fn draw(&self, mut pencil: Pencil) {
        pencil
            .draw_hline('#', Vec2::y(10i32), 80i32)
            .draw_hline('#', Vec2::y(20i32), 80i32);

        for mut entity in self.enemies().iter() {
            let ent_pos: i32 = entity.creature().entity().get_pos();
            let player_pos = self.player().creature().entity().position;

            for i in player_pos - 40..player_pos + 40 {
                if i == ent_pos {
                    pencil.draw_at(entity.creature().entity(), Vec2::xy(40i32 + i, 15i32));
                }
            }
        }
    }
}