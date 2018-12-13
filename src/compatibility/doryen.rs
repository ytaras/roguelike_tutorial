use crate::data::structures::*;
use crate::systems::render::Renderable;
use crate::systems::render::Renderer;
use crate::ui::Game;
use doryen_rs::*;
use specs::prelude::*;

// FIXME find better names
pub struct GameContext<'a, 'b> {
    renderer: DoryenRenderer,
    game: Game<'a, 'b>,
}

impl<'a, 'b> GameContext<'a, 'b> {
    pub fn from_specs(mut w: World) -> Self {
        let console: doryen_rs::Console = w.exec(|level_info: Read<LevelInfo>| {
            doryen_rs::Console::new(level_info.width().into(), level_info.height().into())
        });
        let game = Game::new(w);
        let renderer = DoryenRenderer { console };
        GameContext { game, renderer }
    }
}
struct DoryenRenderer {
    console: Console,
}

impl DoryenRenderer {
    fn blit(&self, root: &mut Console) {
        self.console.blit(0, 0, root, 1., 1., None);
    }
}

impl Renderer for DoryenRenderer {
    fn render<T>(&mut self, pos: Pos, renderable: &T, in_fov: bool)
    where
        T: Renderable,
    {
        let color = if in_fov {
            renderable.color()
        } else {
            renderable.color() * 0.5
        };
        self.console
            .fore(pos.x.into(), pos.y.into(), (color.r, color.g, color.b, 255));
        self.console
            .ascii(pos.x.into(), pos.y.into(), renderable.display_char() as u16);
    }

    fn clear(&mut self) {
        self.console.clear(None, None, None);
    }
}

impl<'a, 'b> Engine for GameContext<'a, 'b> {
    fn update(&mut self, api: &mut DoryenApi) {
        let input = api.input();
        for (key, command) in self.game.key_mapper.commands() {
            if input.key_pressed(key) {
                self.game
                    .game_command_handler
                    .exec(command, &mut self.game.world);
            }
        }
        self.game.update();
    }
    fn render(&mut self, api: &mut DoryenApi) {
        {
            use specs::RunNow;
            self.renderer
                .as_specs_system()
                .run_now(&self.game.world.res);
        }
        self.renderer.blit(api.con());
    }
}
