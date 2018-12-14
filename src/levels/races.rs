use tcod::colors::*;

use crate::data::components::IsFighter;
use crate::data::structures::world_data::MonsterTemplate;
use crate::data::structures::world_data::Race;

pub fn orc_race() -> Race {
    Race {
        default_char: 'o',
        default_color: DESATURATED_GREEN,
        default_fight_skill: IsFighter::new(10, 3, 0),
    }
}

pub fn troll_race() -> Race {
    Race {
        default_char: 'T',
        default_color: DARKER_GREEN,
        default_fight_skill: IsFighter::new(16, 4, 1),
    }
}

pub fn all_monsters() -> Vec<MonsterTemplate> {
    vec![orc_race().to_template(), troll_race().to_template()]
}
