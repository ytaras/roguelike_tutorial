mod console;
mod doryen;

use data::components::IsVisible;
use doryen_rs::DoryenApi;

pub fn render_console() -> console::StdoutRender {
    console::StdoutRender
}

pub fn render_doryen(doryen_api: &mut DoryenApi) -> doryen::DoryenRenderer {
    doryen::DoryenRenderer { doryen_api }
}

type Color = (u8, u8, u8, u8);
pub const RED: Color = (255, 0, 0, 255);
pub const WHITE: Color = (255, 255, 255, 255);
pub const BLACK: Color = (0, 0, 0, 255);

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
