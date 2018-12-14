use crate::systems::render::Color;
use crate::systems::render::Renderable;

#[derive(Copy, Clone, Debug)]
pub struct Race {
    pub default_char: char,
    pub default_color: Color,
}
impl Race {
    pub fn to_template(&self) -> MonsterTemplate {
        MonsterTemplate { race: Some(*self) }
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

#[derive(Copy, Clone, Debug)]
pub struct MonsterTemplate {
    pub race: Option<Race>,
}

const DEFAULT_COLOR: Color = tcod::colors::YELLOW;
const DEFAULT_CHAR: char = '?';

impl Renderable for MonsterTemplate {
    fn color(&self) -> Color {
        self.race.map_or(DEFAULT_COLOR, |r| r.color())
    }

    fn display_char(&self) -> char {
        self.race.map_or(DEFAULT_CHAR, |r| r.display_char())
    }
}
