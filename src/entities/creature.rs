use ruscii::terminal::Color;
use crate::entity::{Entity, TEntity};
use crate::sprites::PLAYER;
use crate::stats::{Stats, TStats};

pub struct Creature {
    entity: Entity,
    stats: Stats
}

impl Creature {
    pub fn new(stats: Stats, entity: Entity) -> Creature {
        Creature { stats, entity }
    }
}

pub trait TCreature {
    fn creature(&mut self) -> &mut Creature;
}

impl TEntity for Creature {
    fn entity(self: &Creature) -> &Entity {
        &self.entity
    }


}

impl TStats for Creature {
    fn stats(&mut self) -> & mut Stats {
        &mut self.stats
    }
}

//Player implementation
//Is a creature, has levels and can move around the map
pub struct Player {
    creature: Creature
}

impl Player {
    pub fn new() -> Player {
        Player{ creature: Creature::new(Stats::default(),
                                        Entity::new("PLayer",
                                                    PLAYER,
                                                    0,
                                                    Color::Magenta))}
    }
}

impl TCreature for Player {
    fn creature(&mut self) -> &mut Creature {
        &mut self.creature
    }
}