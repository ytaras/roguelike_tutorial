mod keymapper;
use self::keymapper::*;
use data::structures::*;
use doryen_rs::*;
use specs::prelude::*;
use systems::render::render_doryen;

pub struct GameWorld {
    world: World,
    key_mapper: KeyMapper,
    game_command_handler: GameCommandHandler,
}

impl GameWorld {
    pub fn new(world: World) -> Self {
        let key_mapper = KeyMapper::new();
        let game_command_handler = GameCommandHandler;
        GameWorld {
            world,
            key_mapper,
            game_command_handler,
        }
    }
}

impl Engine for GameWorld {
    fn update(&mut self, _api: &mut DoryenApi) {
        let input = _api.input();
        for (key, command) in self.key_mapper.commands() {
            if input.key_pressed(key) {
                self.game_command_handler.exec(command);
            }
        }
    }
    fn render(&mut self, api: &mut DoryenApi) {
        let mut renderer = render_doryen(api);
        use specs::RunNow;
        renderer.run_now(&self.world.res);
    }
}

pub struct GameCommandHandler;

impl GameCommandHandler {
    fn exec(&self, gc: &GameCommand) {
        match gc {
            GameCommand::Exit => {
                use std::process::exit;
                exit(0);
            }
        }
    }
}
