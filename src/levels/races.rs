use crate::data::structures::world_data::MonsterTemplate;
use crate::data::structures::world_data::Race;
use tcod::colors::*;

pub const ORC_RACE: Race = Race {
    default_char: 'o',
    default_color: DESATURATED_GREEN,
};

pub const TROLL_RACE: Race = Race {
    default_char: 'T',
    default_color: DARKER_GREEN,
};

pub fn ALL_MONSTERS() -> Vec<MonsterTemplate> {
    vec![ORC_RACE.to_template(), TROLL_RACE.to_template()]
}
