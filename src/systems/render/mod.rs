use data::components::*;
use data::structures::*;
use specs::prelude::*;

pub type Color = (u8, u8, u8, u8);
pub const RED: Color = (255, 0, 0, 255);
pub const WHITE: Color = (255, 255, 255, 255);
pub const BLACK: Color = (0, 0, 0, 255);
pub const DARK_GROUND: Color = (50, 50, 50, 255);
pub const DARK_WALL: Color = (50, 50, 50, 255);
pub const YELLOW: Color = (255, 255, 0, 255);
pub trait Renderable {
    fn color(&self) -> Color;
    fn display_char(&self) -> char;
}

impl Renderable for IsVisible {
    fn color(&self) -> Color {
        self.color
    }
    fn display_char(&self) -> char {
        self.display_char
    }
}

impl Renderable for TileType {
    fn color(&self) -> Color {
        use self::TileType::*;

        match self {
            WALL => DARK_WALL,
            GROUND => DARK_GROUND,
        }
    }

    fn display_char(&self) -> char {
        use self::TileType::*;
        match self {
            WALL => '#',
            GROUND => '.',
        }
    }
}

pub trait Renderer: Sized {
    fn render<T>(&mut self, pos: Pos, r: &T)
    where
        T: Renderable;

    fn as_specs_system(&mut self) -> RenderWrapper<Self> {
        RenderWrapper(self)
    }
}

pub struct RenderWrapper<'a, R: Renderer + 'a>(&'a mut R);

impl<'a, R> System<'a> for RenderWrapper<'a, R>
where
    R: Renderer,
{
    type SystemData = (
        ReadStorage<'a, HasPos>,
        ReadStorage<'a, IsVisible>,
        Read<'a, LevelInfo>,
    );
    fn run(&mut self, (pos, vis, li): Self::SystemData) {
        use specs::Join;
        let x: &mut R = &mut self.0;
        for (pos, vis) in li.all_cells() {
            x.render(pos, vis);
        }
        for (pos, vis) in (&pos, &vis).join() {
            x.render(pos.0, vis);
        }
    }
}
