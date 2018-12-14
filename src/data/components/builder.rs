use specs::prelude::*;

use crate::data::components::*;
use crate::data::structures::world_data::MonsterTemplate;
use crate::data::structures::Pos;
use crate::systems::render::Color;
use crate::systems::render::Renderable;

pub trait RichEntityBuilder
where
    Self: Sized,
{
    fn with_actor_components(self, display_char: char, color: Color, pos: Pos) -> Self;
    fn is_player(self) -> Self;
    fn with_ai(self) -> Self;
    fn with_fighter(self, f: IsFighter) -> Self;

    fn is_monster(self, mt: &MonsterTemplate, pos: Pos) -> Self {
        let display_char = mt.display_char();
        let color = mt.color();
        self.with_actor_components(display_char, color, pos)
            .with_fighter(mt.fight_skills().clone())
            .with_ai()
    }
}

impl<'a> RichEntityBuilder for EntityBuilder<'a> {
    fn with_actor_components(self, display_char: char, color: Color, pos: Pos) -> Self {
        self.with(HasPos(pos)).with(IsVisible {
            color,
            display_char,
        })
    }

    fn is_player(self) -> Self {
        self.with(IsPlayer)
            .with(HasVision::new(8))
            .with(IsFighter::new(30, 5, 2))
    }

    fn with_ai(self) -> Self {
        self.with(HasBrain::bored())
    }

    fn with_fighter(self, f: IsFighter) -> Self {
        self.with(f)
    }
}
