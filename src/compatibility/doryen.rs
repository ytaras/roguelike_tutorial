use data::structures::*;
use doryen_rs::*;
use specs::prelude::*;
use systems::logic::ExecuteCommands;
use systems::render::Renderable;
use systems::render::Renderer;
use ui::Game;

// FIXME find better names
pub struct GameContext {
    renderer: DoryenRenderer,
    game: Game,
}

impl GameContext {
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
    fn render<T>(&mut self, pos: Pos, r: &T)
    where
        T: Renderable,
    {
        self.console.fore(pos.x.into(), pos.y.into(), r.color());
        self.console
            .ascii(pos.x.into(), pos.y.into(), r.display_char() as u16);
    }
}

impl Engine for GameContext {
    fn update(&mut self, api: &mut DoryenApi) {
        use specs::RunNow;
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
