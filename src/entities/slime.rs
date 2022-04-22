use ruscii::terminal::Color;
use crate::entities::creature::{Creature, TCreature};
use crate::Entity;
use crate::sprites::SLIME;
use crate::stats::Stats;

pub struct Slime {
    creature: Creature
}

impl Slime {
    pub fn new(pos: i32) -> Slime {
        Slime{ creature: Creature::new(Stats::default(),
                                        Entity::new("Slime",
                                                    SLIME,
                                                    pos,
                                                    Color::Green))}
    }
}

impl TCreature for Slime {
    fn creature(&mut self) -> &mut Creature {
        &mut self.creature
    }
}