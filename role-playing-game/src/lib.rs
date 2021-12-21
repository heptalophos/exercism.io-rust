#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
            level: level
        }
    }

    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 { return None }
        Some(Player::new(self.level))
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            if mana_cost <= self.health {
                self.health -= mana_cost;
            } else {
                self.health = 0;
            }
            return 0;
        }
        if Some(mana_cost) > self.mana {
            return 0;
        }
        self.mana = Some(self.mana.unwrap() - mana_cost);
        return 2 * mana_cost
    }
}
