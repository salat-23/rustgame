use std::fs::read_to_string;

pub enum StatsType {
    Player,
    Slime,
}

pub struct BaseStats {
    base_hp: i32,
    base_def: i32,
    base_atk: i32,
}

impl BaseStats {
    pub fn get_for_type(s_type: StatsType) -> BaseStats {
        match s_type {
            StatsType::Player => {
                BaseStats { base_hp: 3, base_def: 2, base_atk: 1 }
            }
            _ => BaseStats { base_hp: 1, base_def: 1, base_atk: 1 }
        }
    }
}

pub struct Stats {
    hp: i32,
    def: i32,
    atk: i32,
    lvl: i32,
    exp: i32,
}

impl Stats {
    pub fn default() -> Self {
        Stats { hp: 10, def: 5, atk: 3, lvl: 1, exp: 0 }
    }

    pub fn generate(lvl: i32, base_stats: BaseStats) -> Stats {
        Stats {
            hp: ((base_stats.base_hp * lvl) as f64 / (lvl as f64 * 1.5)) as i32,
            def: (base_stats.base_def * lvl / (lvl * 4 / 3 + base_stats.base_def / 2)),
            atk: (base_stats.base_atk * lvl / (lvl * 4 / 3)),
            lvl,
            exp: 0
        }
    }
}

pub trait TStats {
    fn stats(&mut self) -> &mut Stats;
}