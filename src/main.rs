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

    let (level, room) = level_1(&mut rand::thread_rng());
    let player_pos = room.center();

    world.add_resource(level);

    world
        .create_entity()
        .is_player()
        .with_actor_components('@', RED, player_pos)
        .build();

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
