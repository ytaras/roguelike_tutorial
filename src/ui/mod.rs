use doryen_rs::*;
use specs::prelude::*;

use data::components::*;
use data::structures::*;
use systems::logic::*;
use systems::render::render_doryen;

use self::keymapper::*;

mod keymapper;

pub struct GameWorld {
    world: World,
    key_mapper: KeyMapper,
    game_command_handler: GameCommandHandler,
    console: Console,
}

impl GameWorld {
    pub fn new(mut world: World) -> Self {
        let key_mapper = KeyMapper::new();
        let game_command_handler = GameCommandHandler;
        let console: Console = world.exec(|level_info: Read<LevelInfo>| {
            Console::new(level_info.width().into(), level_info.height().into())
        });
        GameWorld {
            world,
            key_mapper,
            game_command_handler,
            console,
        }
    }
}

impl Engine for GameWorld {
    fn update(&mut self, api: &mut DoryenApi) {
        use specs::RunNow;
        let input = api.input();
        for (key, command) in self.key_mapper.commands() {
            if input.key_pressed(key) {
                self.game_command_handler.exec(command, &mut self.world);
            }
        }
        // TODO - Use dispatcher
        ExecuteCommands.run_now(&self.world.res);
        self.world.maintain();
    }
    fn render(&mut self, api: &mut DoryenApi) {
        let mut renderer = render_doryen(api.con());
        use specs::RunNow;
        renderer.run_now(&self.world.res);
    }
}

pub struct GameCommandHandler;

impl GameCommandHandler {
    fn exec(&self, gc: &Command, world: &mut World) {
        match gc {
            Command::GameCommand(GameCommand::Exit) => {
                use std::process::exit;
                exit(0);
            }
            Command::PlayerCommand(ac) => {
                use specs::RunNow;
                // TODO Extract command handling from UI layer
                let mut system: AssertUnique<IsPlayer> = Default::default();
                system.run_now(&world.res);
                // TODO Extract exec to system or provide helper methods - to decide
                let (e, ispl, mut pl): (
                    Entities,
                    ReadStorage<IsPlayer>,
                    WriteStorage<PlansExecuting>,
                ) = world.system_data();
                use specs::Join;
                for (e, _) in (&e, &ispl).join() {
                    pl.insert(e, PlansExecuting::new(ac.to_owned())).unwrap();
                }
            }
        }
    }
}
