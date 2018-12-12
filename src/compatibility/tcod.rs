use crate::data::structures::Pos;
use crate::systems::render::Color;
use crate::systems::render::{Renderable, Renderer};
use tcod::colors::YELLOW;
use tcod::BackgroundFlag;

impl<'a, C: ::tcod::Console> Renderer for C {
    fn render<T>(&mut self, pos: Pos, r: &T, in_fov: bool)
    where
        T: Renderable,
    {
        let mut color = r.color();
        if !in_fov {
            color = color * 0.5;
        }
        self.set_default_foreground(color);
        self.put_char(
            pos.x.into(),
            pos.y.into(),
            r.display_char(),
            BackgroundFlag::None,
        );
    }

    fn clear(&mut self) {
        self.clear();
    }
}
