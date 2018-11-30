use specs::prelude::*;

use data::components::*;
use data::structures::Pos;

pub trait RichEntityBuilder {
    fn with_actor_components(&self, display_char: char, color: Color, pos: Pos) -> Self;
    fn is_player(&self) -> Self;
}

impl RichEntityBuilder for EntityBuilder {
    fn with_actor_components(&self, display_char: char, color: Color, pos: Pos) -> Self {
        self.with(pos).with(IsVisible { color, display_char })
    }

    fn is_player(&self) -> Self {
        self.with(IsPlayer)
    }
}