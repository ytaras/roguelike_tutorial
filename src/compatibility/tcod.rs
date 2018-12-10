use crate::data::structures::Pos;
use crate::systems::render::Color;
use crate::systems::render::{Renderable, Renderer};
use tcod::BackgroundFlag;

impl<'a, C: ::tcod::Console> Renderer for C {
    fn render<T>(&mut self, pos: Pos, r: &T)
    where
        T: Renderable,
    {
        self.set_default_foreground(color_to_tcod(r.color()));
        self.put_char(
            pos.x.into(),
            pos.y.into(),
            r.display_char(),
            BackgroundFlag::None,
        );
    }
}

pub fn color_to_tcod((r, g, b, _): Color) -> ::tcod::Color {
    ::tcod::Color { r, g, b }
}
