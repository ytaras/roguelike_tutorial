use crate::common::query::singleton;
use crate::data::components::*;
use crate::data::structures::matrix::Matrix;
use crate::data::structures::*;
use log::warn;
use specs::prelude::*;

pub type Color = ::tcod::Color;
pub const RED: Color = ::tcod::colors::RED;
pub const LIGHT_GROUND: Color = tcod::colors::GREY;
pub const LIGHT_WALL: Color = tcod::colors::GREY;
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
            Wall => LIGHT_WALL,
            RoomWall => LIGHT_WALL,
            Ground => LIGHT_GROUND,
        }
    }

    fn display_char(&self) -> char {
        use self::TileType::*;
        match self {
            Wall => '#',
            RoomWall => '±',
            Ground => '.',
        }
    }
}

pub trait Renderer: Sized {
    fn render<T>(&mut self, pos: Pos, r: &T, in_fov: bool)
    where
        T: Renderable;

    fn clear(&mut self);

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
        ReadStorage<'a, HasVision>,
        ReadStorage<'a, IsPlayer>,
        Read<'a, LevelInfo>,
    );
    fn run(&mut self, (pos, vis, vision, is_pl, li): Self::SystemData) {
        use specs::Join;

        let (player_vision, _) = singleton((&vision, &is_pl)).unwrap();
        let pw: Option<&Matrix<bool>> = player_vision.fov();
        let mem: Option<&Matrix<bool>> = player_vision.memory();

        if let (Some(fov), Some(mem)) = (pw, mem) {
            let x: &mut R = &mut self.0;
            x.clear();
            for (pos, vis) in li.all_cells() {
                let show = mem[pos];
                if show {
                    x.render(pos, vis, fov[pos]);
                }
            }
            for (pos, vis) in (&pos, &vis).join() {
                if fov[pos.0] {
                    x.render(pos.0, vis, fov[pos.0]);
                }
            }
        } else {
            warn!("Not found player FOV, not rendering");
        }
    }
}
