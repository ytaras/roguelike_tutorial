use doryen_rs::Console;

use data::components::IsVisible;
use data::structures::TileType;

mod doryen;

pub fn render_doryen(doryen_api: &mut Console) -> doryen::DoryenRenderer {
    doryen::DoryenRenderer { doryen_api }
}

type Color = (u8, u8, u8, u8);
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
