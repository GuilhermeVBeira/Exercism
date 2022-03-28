// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const MINIMUM_LEVEL_FOR_MANA_USE: u32 = 10;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut mana = None;
            if self.level >= MINIMUM_LEVEL_FOR_MANA_USE {
                mana = Some(100);
            };

            return Some(Player {
                health: 100,
                mana,
                level: self.level,
            });
        } else {
            None
        }
    }

    fn validate_mana_level(&mut self, mana_cost: u32) {
        if self.level <= MINIMUM_LEVEL_FOR_MANA_USE {
            if mana_cost > self.health {
                self.health = 0
            } else {
                self.health -= mana_cost;
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);
                return mana_cost * 2;
            }
            self.validate_mana_level(mana_cost);
            return 0;
        } else {
            self.validate_mana_level(mana_cost);
            return 0;
        }
    }
}
