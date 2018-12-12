extern crate doryen_rs;
extern crate rogue_tutorial;
extern crate specs;

use doryen_rs::{App, AppOptions};
use specs::prelude::*;

use rogue_tutorial::compatibility::doryen::*;
use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::levels::level_1;
use rogue_tutorial::systems::render::*;
use tcod::colors::RED;

const CONSOLE_DIM: Dim = Dim {
    width: 90,
    height: 50,
};

fn main() {
    let mut world = World::new();
    world.register::<HasPos>();
    world.register::<IsVisible>();
    world.register::<IsPlayer>();
    world.register::<PlansExecuting>();
    world.register::<TakesWholeTile>();
    world.register::<HasVision>();

    let mut rng = rand::thread_rng();
    // FXIME Extract to script
    let (level_info, level) = level_1(&mut rng);

    world.add_resource(level_info);

    world
        .create_entity()
        .is_player()
        .with_actor_components('@', RED, level.player_pos)
        .build();

    for (monster, pos) in level.monsters {
        world.create_entity().is_monster(&monster, pos).build();
    }

    let mut app = App::new(AppOptions {
        console_width: CONSOLE_DIM.width.into(),
        console_height: CONSOLE_DIM.height.into(),
        screen_width: CONSOLE_DIM.width as u32 * 8,
        screen_height: CONSOLE_DIM.height as u32 * 8,
        window_title: "my roguelike".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(GameContext::from_specs(world)));
    app.run();
}
