use specs::prelude::*;

use crate::common::validations::Validation;
use crate::data::components::*;
use crate::data::structures::*;
use crate::systems::logic::*;
use crate::systems::render::Renderer;
use crate::systems::validation::MoveValidation;

use self::keymapper::*;

mod keymapper;

pub struct Game<'a, 'b> {
    pub world: World,
    pub key_mapper: KeyMapper,
    pub game_command_handler: GameCommandHandler,
    tick_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(mut world: World) -> Self {
        let key_mapper = KeyMapper::default();
        let game_command_handler = GameCommandHandler;

        let mut tick_dispatcher = DispatcherBuilder::new()
            .with(GetAiCommand, "ai_decide", &[])
            .with(ExecuteCommands, "execute_commands", &["ai_decide"])
            .with(Fov::default(), "fov", &[])
            .with(ExecuteDamage, "execute_damage", &["execute_commands"])
            .build();

        tick_dispatcher.setup(&mut world.res);
        game_command_handler.setup(&mut world);

        Game {
            world,
            key_mapper,
            game_command_handler,
            tick_dispatcher,
        }
    }

    pub fn render_on<R: Renderer>(&mut self, r: &mut R) {
        use specs::RunNow;
        r.as_specs_system().run_now(&self.world.res);
    }

    pub fn update(&mut self) {
        self.tick_dispatcher.dispatch(&self.world.res);
        self.world.maintain();
    }
}

pub struct GameCommandHandler;

impl GameCommandHandler {
    pub fn setup(&self, res: &mut World) {
        res.register::<IsPlayer>();
        res.register::<TakesWholeTile>();
    }

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
                    ActorCommand::MeleeAttack { .. } => unreachable!(),
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
                                pl.insert(e, PlansExecuting::new(res)).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
