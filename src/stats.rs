pub struct Stats {
    hp: i32,
    def: i32,
    atk: i32
}

impl Stats {
    pub fn default() -> Self {
        Stats{ hp: 10, def: 5, atk: 3 }
    }
}

pub trait TStats {
    fn stats(&mut self) -> &mut Stats;
}