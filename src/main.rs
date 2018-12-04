extern crate doryen_rs;
extern crate rogue_tutorial;
extern crate specs;

use doryen_rs::{App, AppOptions};
use specs::prelude::*;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::levels::level_1;
use rogue_tutorial::systems::render::*;
use rogue_tutorial::ui::GameWorld;

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

    let level = level_1();
    let player_pos = Pos {
        x: level.width() / 2,
        y: level.height() / 2,
    };

    world.add_resource(level);

    world
        .create_entity()
        .is_player()
        .with_actor_components('@', RED, player_pos)
        .build();

    let mut npc_pos = player_pos;
    npc_pos.x -= 5;
    world
        .create_entity()
        .with_actor_components('@', YELLOW, npc_pos)
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
    app.set_engine(Box::new(GameWorld::new(world)));
    app.run();
}
