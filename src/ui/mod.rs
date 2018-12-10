use specs::prelude::*;

use crate::common::validations::Validation;
use crate::data::components::*;
use crate::data::structures::*;
use crate::systems::logic::*;
use crate::systems::validation::MoveValidation;

use self::keymapper::*;
use crate::systems::render::Renderer;

mod keymapper;

pub struct Game {
    pub world: World,
    pub key_mapper: KeyMapper,
    pub game_command_handler: GameCommandHandler,
}

impl Game {
    pub fn new(world: World) -> Self {
        let key_mapper = KeyMapper::new();
        let game_command_handler = GameCommandHandler;

        Game {
            world,
            key_mapper,
            game_command_handler,
        }
    }

    pub fn render_on<R: Renderer>(&mut self, r: &mut R) {
        use specs::RunNow;
        r.as_specs_system().run_now(&self.world.res);
    }

    pub fn update(&mut self) {
        use specs::RunNow;
        ExecuteCommands.run_now(&mut self.world.res);
        self.world.maintain();
    }
}

pub struct GameCommandHandler;

impl GameCommandHandler {
    // FIXME - Use validation framework for everyone here
    pub fn exec(&self, gc: &Command, world: &mut World) {
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

                match ac {
                    ActorCommand::Move(dir) => {
                        if let Some(res) = MoveValidation::default().exec(*dir, world) {
                            // TODO Extract exec to system or provide helper methods - to decide
                            let (e, ispl, mut pl): (
                                Entities,
                                ReadStorage<IsPlayer>,
                                WriteStorage<PlansExecuting>,
                            ) = world.system_data();
                            use specs::Join;
                            for (e, _) in (&e, &ispl).join() {
                                pl.insert(e, PlansExecuting::new(ActorCommand::Move(res)))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
