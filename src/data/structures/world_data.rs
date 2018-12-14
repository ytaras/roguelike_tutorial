use crate::data::components::IsFighter;
use crate::systems::render::Color;
use crate::systems::render::Renderable;

#[derive(Clone, Debug)]
pub struct Race {
    pub default_char: char,
    pub default_color: Color,
    pub default_fight_skill: IsFighter,
}
impl Race {
    pub fn to_template(&self) -> MonsterTemplate {
        MonsterTemplate {
            race: Some(self.clone()),
        }
    }
}
impl Renderable for Race {
    fn color(&self) -> Color {
        self.default_color
    }

    fn display_char(&self) -> char {
        self.default_char
    }
}

#[derive(Clone, Debug)]
pub struct MonsterTemplate {
    pub race: Option<Race>,
}

const DEFAULT_COLOR: Color = tcod::colors::YELLOW;
const DEFAULT_CHAR: char = '?';

impl Renderable for MonsterTemplate {
    fn color(&self) -> Color {
        self.race.as_ref().map_or(DEFAULT_COLOR, |r| r.color())
    }

    fn display_char(&self) -> char {
        self.race
            .as_ref()
            .map_or(DEFAULT_CHAR, |r| r.display_char())
    }
}

impl MonsterTemplate {
    pub fn fight_skills(&self) -> &IsFighter {
        &self.race.as_ref().unwrap().default_fight_skill
    }
}
